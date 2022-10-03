use crate::file_info::FileInfo;
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{DefinerUsage, IfStatement};
use crate::parser::types::objects::conversion::{get_container, get_definer};
use crate::parser::types::sizes::Sizes;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::Tags;
use crate::parser::types::ty::Type;
use crate::parser::types::ArrayType;
use crate::{ContainerType, DefinerType};

#[derive(Debug, Clone)]
pub struct ParsedContainer {
    pub name: String,
    pub object_type: ContainerType,
    pub members: Vec<StructMember>,
    pub tags: Tags,
    pub file_info: FileInfo,
}

impl ParsedContainer {
    pub fn new(
        name: &str,
        members: Vec<StructMember>,
        tags: Tags,
        object_type: ContainerType,
        file_info: FileInfo,
    ) -> Self {
        let mut s = Self {
            name: name.to_string(),
            object_type,
            members,
            tags,
            file_info,
        };

        s.self_check();
        s.set_used_in_if();
        s.set_if_statements();

        s
    }

    pub fn contains_definer(&self, ty_name: &str) -> DefinerUsage {
        fn inner(m: &StructMember, ty_name: &str, variable_name: &str) -> DefinerUsage {
            match m {
                StructMember::Definition(d) => {
                    if let Type::Identifier { s, .. } = d.ty() {
                        if s == ty_name {
                            return DefinerUsage::NotInIf;
                        }
                    }
                }
                StructMember::IfStatement(statement) => {
                    if statement.name() == variable_name {
                        return DefinerUsage::InIf;
                    }

                    let mut not_in_if = false;
                    for m in statement.all_members() {
                        match inner(m, ty_name, variable_name) {
                            DefinerUsage::Unused => {}
                            DefinerUsage::NotInIf => not_in_if = true,
                            DefinerUsage::InIf => return DefinerUsage::InIf,
                        }
                    }

                    if not_in_if {
                        return DefinerUsage::NotInIf;
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    let mut not_in_if = false;

                    for m in optional.members() {
                        match inner(m, ty_name, variable_name) {
                            DefinerUsage::Unused => {}
                            DefinerUsage::NotInIf => not_in_if = true,
                            DefinerUsage::InIf => return DefinerUsage::InIf,
                        }
                    }

                    if not_in_if {
                        return DefinerUsage::NotInIf;
                    }
                }
            }

            DefinerUsage::Unused
        }

        let variable_name = self.get_variable_name_of_definer_ty(ty_name);

        if let Some(variable_name) = variable_name {
            let mut not_in_if = false;

            for m in self.fields() {
                match inner(m, ty_name, &variable_name) {
                    DefinerUsage::Unused => {}
                    DefinerUsage::NotInIf => not_in_if = true,
                    DefinerUsage::InIf => return DefinerUsage::InIf,
                }
            }

            if not_in_if {
                return DefinerUsage::NotInIf;
            }
        }

        DefinerUsage::Unused
    }

    pub fn get_variable_name_of_definer_ty(&self, ty_name: &str) -> Option<String> {
        for d in self.all_definitions() {
            if let Type::Identifier { s, .. } = d.ty() {
                if s == ty_name {
                    return Some(d.name().to_string());
                }
            }
        }

        None
    }

    pub fn get_field_ty(&self, field_name: &str) -> &Type {
        for d in self.all_definitions() {
            if d.name() == field_name {
                return d.ty();
            }
        }

        panic!("unable to find field: '{}'", field_name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn all_definitions(&self) -> Vec<&StructMemberDefinition> {
        fn inner<'a>(m: &'a StructMember, v: &mut Vec<&'a StructMemberDefinition>) {
            match m {
                StructMember::Definition(d) => v.push(d),
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members() {
                        inner(m, v);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, v);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.fields() {
            inner(m, &mut v);
        }

        v
    }

    fn add_sizes_values(
        e: &Self,
        m: &StructMember,
        containers: &[Self],
        definers: &[Definer],
        sizes: &mut Sizes,
    ) {
        match m {
            StructMember::Definition(d) => *sizes += d.ty().sizes_parsed(e, containers, definers),
            StructMember::OptionalStatement(optional) => {
                let minimum = sizes.minimum();

                for m in optional.members() {
                    Self::add_sizes_values(e, m, containers, definers, sizes);
                }

                // The optional statement doesn't have be be here, so the minimum doesn't get incremented
                sizes.set_minimum(minimum);
            }
            StructMember::IfStatement(statement) => {
                let statement_sizes = Self::get_complex_sizes(statement, e, containers, definers);

                *sizes += statement_sizes;
            }
        }
    }

    pub fn create_sizes(&self, containers: &[Self], definers: &[Definer]) -> Sizes {
        let mut sizes = Sizes::new();
        for m in self.fields() {
            Self::add_sizes_values(self, m, containers, definers, &mut sizes);
        }

        sizes
    }

    pub fn get_complex_sizes(
        statement: &IfStatement,
        e: &Self,
        containers: &[Self],
        definers: &[Definer],
    ) -> Sizes {
        let mut if_sizes = Sizes::new();

        for m in statement.members() {
            Self::add_sizes_values(e, m, containers, definers, &mut if_sizes);
        }

        let mut smallest_sizes = if_sizes;
        let mut largest_sizes = if_sizes;

        let mut else_if_sizes;

        for elseif in statement.else_ifs() {
            else_if_sizes = Sizes::new();

            for m in elseif.members() {
                Self::add_sizes_values(e, m, containers, definers, &mut else_if_sizes);
            }

            if else_if_sizes.minimum() < smallest_sizes.minimum() {
                smallest_sizes = else_if_sizes;
            }
            if else_if_sizes.maximum() > largest_sizes.maximum() {
                largest_sizes = else_if_sizes;
            }
        }

        else_if_sizes = Sizes::new();
        for m in statement.else_members() {
            Self::add_sizes_values(e, m, containers, definers, &mut else_if_sizes);
        }

        if else_if_sizes.minimum() < smallest_sizes.minimum() {
            smallest_sizes = else_if_sizes;
        }
        if else_if_sizes.maximum() > largest_sizes.maximum() {
            largest_sizes = else_if_sizes;
        }

        let mut sizes = Sizes::new();
        sizes.set_minimum(smallest_sizes.minimum());
        sizes.set_maximum(largest_sizes.maximum());
        sizes
    }

    pub fn get_type_of_variable(&self, variable_name: &str) -> Type {
        for d in self.all_definitions() {
            if d.name() == variable_name {
                return d.ty().clone();
            }
        }

        panic!("unable to find type {}", variable_name)
    }

    pub fn fields(&self) -> &[StructMember] {
        self.members.as_slice()
    }

    pub fn fields_mut(&mut self) -> &mut [StructMember] {
        self.members.as_mut_slice()
    }

    pub fn recursive_only_has_io_errors(&self, containers: &[Self], definers: &[Definer]) -> bool {
        if self.contains_string_or_cstring() {
            return false;
        }

        for t in self.get_types_needing_import() {
            if let Some(d) = get_definer(definers, t.as_str(), self.tags()) {
                if d.definer_ty() == DefinerType::Enum {
                    return false;
                }
            } else if let Some(c) = get_container(containers, t.as_str(), self.tags()) {
                if !c.recursive_only_has_io_errors(containers, definers) {
                    return false;
                }
            } else {
                unreachable!()
            }
        }

        true
    }

    pub fn contains_string_or_cstring(&self) -> bool {
        for d in self.all_definitions() {
            match d.ty() {
                Type::CString | Type::String { .. } | Type::SizedCString => return true,
                Type::Array(array) => {
                    if matches!(array.ty(), ArrayType::CString) {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    pub fn get_types_needing_import(&self) -> Vec<String> {
        self.get_complex_types()
    }

    fn get_complex_types(&self) -> Vec<String> {
        let mut v = Vec::new();

        for d in self.all_definitions() {
            match &d.struct_type() {
                Type::Array(a) => {
                    if let ArrayType::Complex(i) = a.ty() {
                        v.push(i.clone());
                    }
                }
                Type::Identifier { s, .. } => {
                    v.push(s.clone());
                }
                _ => {}
            }
        }

        v.sort_unstable();
        v.dedup();

        v
    }

    pub fn set_used_in_if(&mut self) {
        let mut variables_used_in_if = Vec::new();

        fn find_used_in_if(m: &StructMember, variables_used_in_if: &mut Vec<String>) {
            match m {
                StructMember::Definition(_) => {}
                StructMember::IfStatement(statement) => {
                    variables_used_in_if.push(statement.name().to_string());

                    for m in statement.all_members() {
                        find_used_in_if(m, variables_used_in_if);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        find_used_in_if(m, variables_used_in_if);
                    }
                }
            }
        }

        for m in self.fields() {
            find_used_in_if(m, &mut variables_used_in_if);
        }

        for d in self.all_definitions_mut() {
            d.set_used_in_if(variables_used_in_if.contains(&d.name().to_string()));
        }
    }

    pub fn all_definitions_mut(&mut self) -> Vec<&mut StructMemberDefinition> {
        fn inner<'a>(m: &'a mut StructMember, v: &mut Vec<&'a mut StructMemberDefinition>) {
            match m {
                StructMember::Definition(d) => v.push(d),
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members_mut() {
                        inner(m, v);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members_mut() {
                        inner(m, v);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.fields_mut() {
            inner(m, &mut v);
        }

        v
    }

    fn set_if_statements(&mut self) {
        fn inner(m: &mut StructMember, c: &ParsedContainer) {
            match m {
                StructMember::Definition(_) => {}
                StructMember::IfStatement(statement) => {
                    statement.set_original_ty(c.get_type_of_variable(statement.name()));

                    for else_if in statement.else_ifs_mut() {
                        else_if.set_original_ty(c.get_type_of_variable(else_if.name()));
                    }

                    for m in statement.all_members_mut() {
                        inner(m, c);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members_mut() {
                        inner(m, c);
                    }
                }
            }
        }

        let c = self.clone();
        for m in &mut self.members {
            inner(m, &c);
        }
    }

    pub fn self_check(&self) {
        self.panic_on_duplicate_field_names();
    }

    pub fn panic_on_duplicate_field_names(&self) {
        let mut v = Vec::new();

        for d in self.all_definitions() {
            v.push(d.name());
        }
        v.sort_unstable();

        let mut previous_name = "";
        for e in v {
            if e == previous_name {
                panic!(
                    "struct '{struct_name}' contains duplicate fields '{field_name}'",
                    struct_name = self.name(),
                    field_name = e
                );
            }
            previous_name = e;
        }
    }
}