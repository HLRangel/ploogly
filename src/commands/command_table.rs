use std::collections::HashMap;
use std::io::ErrorKind;

/// Describes a template command so it can be dispatched and documented.
pub struct CommandDescriptor {
    pub name: &'static str,
    pub signature: &'static str,
    pub description: &'static str,

    /// The function that implements the command.
    /// It receives the same arguments as the current dispatch match arms.
    pub handler: fn(
        origin: &[u8],
        result: &mut Vec<u8>,
        last: &mut usize,
        current: &mut usize,
        vars: &mut HashMap<String, Vec<u8>>,
        anon_stack: &mut Vec<Vec<u8>>,
    ) -> Result<(), std::io::Error>,
}

/// All available template commands, in the order they should be tried.
pub static COMMANDS: &[CommandDescriptor] = &[
    CommandDescriptor {
        name: "include",
        signature: "<path>",
        description: "Process and include an external template",
        handler: |origin, result, last, current, vars, anon_stack| {
            crate::commands::include::include(result, origin, last, current, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "var",
        signature: "<name>",
        description: "Insert variable value; newlines become <br>",
        handler: |origin, result, last, current, vars, anon_stack| {
            crate::commands::var::var(result, origin, last, current, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "set",
        signature: "<name> <value>",
        description: "Assign a variable",
        handler: |origin, result, last, current, vars, anon_stack| {
            crate::commands::set::set(origin, current, last, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "unset",
        signature: "<name>",
        description: "Remove a variable",
        handler: |origin, result, last, current, vars, _anon_stack| {
            crate::commands::unset::unset(origin, last, current, vars)
        },
    },
    CommandDescriptor {
        name: "ifdef",
        signature: "<var>",
        description: "Show inner block if variable exists",
        handler: |origin, result, last, current, vars, anon_stack| {
            let mut tores = crate::commands::if_n_def::ifdef(origin, current, last, vars, anon_stack)?;
            result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "ifndef",
        signature: "<var>",
        description: "Show inner block if variable does NOT exist",
        handler: |origin, result, last, current, vars, anon_stack| {
            let mut tores = crate::commands::if_n_def::ifndef(origin, current, last, vars, anon_stack)?;
            result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "template",
        signature: "<path> [var# val ...]",
        description: "Include a template with local variables",
        handler: |origin, result, last, current, vars, anon_stack| {
            let mut tores = crate::commands::template::template(origin, last, current, vars, anon_stack)?;
            result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "truncate",
        signature: "<text> <length>",
        description: "Truncate string to given length and commute with \"...\"",
        handler: |origin, result, last, current, vars, anon_stack| {
            let mut tores = crate::commands::truncate::truncate(origin, last, current, vars, anon_stack)?;
            result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "redir",
        signature: "<url>",
        description: "Create a redirect page",
        handler: |origin, _result, last, current, vars, _anon_stack| {
            crate::commands::redir::redir(origin, last, current, vars)
        },
    },
    CommandDescriptor {
        name: "iter_dir",
        signature: "<dir>",
        description: "Iterate directory files; inner block runs per file",
        handler: |origin, result, last, current, vars, anon_stack| {
            let mut tores = crate::commands::iter_dir::iter_dir(origin, last, current, vars, anon_stack)?;
            result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "add_document",
        signature: "<docpath> <basename>",
        description: "Add document to base",
        handler: |origin, _result, last, current, vars, anon_stack| {
            crate::commands::add_document::add_document(origin, last, current, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "produce_base",
        signature: "<basename>",
        description: "Produce all entries in base",
        handler: |origin, _result, last, current, vars, anon_stack| {
            crate::commands::produce_base::produce_base_cmd(origin, last, current, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "load_base",
        signature: "<basename>",
        description: "Load base as JSON string",
        handler: |origin, result, last, current, _vars, _anon_stack| {
            crate::commands::load_base::load_base(result, origin, last, current)
        },
    },
    CommandDescriptor {
        name: "cutbase_extension",
        signature: "<ext> <base>",
        description: "Remove entries not ending with .<ext>",
        handler: |origin, result, last, current, vars, anon_stack| {
            crate::commands::cutbase_extension::cutbase_extension(origin, result, current, last, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "sortbase_by_key",
        signature: "<key> <base>",
        description: "Sort entries by context key",
        handler: |origin, result, last, current, vars, anon_stack| {
            crate::commands::sortbase_by_key::sortbase_by_key(origin, result, current, last, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "reverse_base_order",
        signature: "<base>",
        description: "Reverse entry order",
        handler: |origin, result, last, current, vars, anon_stack| {
            crate::commands::reverse_base_order::reverse_base_order(origin, result, current, last, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "iter_base",
        signature: "<base>",
        description: "Iterate base entries; inner block gets entry data",
        handler: |origin, result, last, current, vars, anon_stack| {
            crate::commands::iter_base::iter_base(origin, result, last, current, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "gen_doc_from_template",
        signature: "<output> <template>",
        description: "Generate document from template",
        handler: |origin, _result, last, current, vars, anon_stack| {
            crate::commands::gen_doc::gen_doc_from_template(origin, last, current, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "ltrim",
        signature: "<string> <count>",
        description: "Left-trim <count> characters",
        handler: |origin, result, last, current, vars, anon_stack| {
            crate::commands::ltrim::ltrim(origin, result, current, last, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "rtrim",
        signature: "<string> <count>",
        description: "Right-trim <count> characters",
        handler: |origin, result, last, current, vars, anon_stack| {
            crate::commands::rtrim::rtrim(origin, result, current, last, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "call",
        signature: "...",
        description: "Print evaluated arguments (debug)",
        handler: |origin, _result, last, current, vars, anon_stack| {
            crate::commands::call::call(origin, current, last, vars, anon_stack)
        },
    },
    CommandDescriptor {
        name: "macro",
        signature: "<name> <params>",
        description: "Define macro (debug only)",
        handler: |origin, _result, last, current, vars, anon_stack| {
            crate::commands::create_macro::create_macro(origin, current, last, vars, anon_stack)?;
            Ok(())
        },
    },
    CommandDescriptor {
        name: "append_data_to_file",
        signature: "<filename> <inner>",
        description: "Append processed inner content to file in output directory",
        handler: |origin, _result, last, current, vars, anon_stack| {
            crate::commands::append_data_to_file::append_data_to_file(origin, last, current, vars, anon_stack)
        },
    },
];
