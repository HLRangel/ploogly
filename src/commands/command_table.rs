use super::CommandContext;

/// Describes a template command so it can be dispatched and documented.
pub struct CommandDescriptor {
    pub name: &'static str,
    pub signature: &'static str,
    pub description: &'static str,
    /// The function that implements the command.
    pub handler: fn(ctx: &mut CommandContext) -> Result<(), std::io::Error>,
}

/// All available template commands, in the order they should be tried.
pub static COMMANDS: &[CommandDescriptor] = &[
    CommandDescriptor {
        name: "include",
        signature: "<path>",
        description: "Process and include an external template",
        handler: |ctx| {
            crate::commands::include::include(ctx)
        },
    },
    CommandDescriptor {
        name: "var",
        signature: "<name>",
        description: "Insert variable value; newlines become <br>",
        handler: |ctx| {
            crate::commands::var::var(ctx)
        },
    },
    CommandDescriptor {
        name: "set",
        signature: "<name> <value>",
        description: "Assign a variable",
        handler: |ctx| {
            crate::commands::set::set(ctx)
        },
    },
    CommandDescriptor {
        name: "unset",
        signature: "<name>",
        description: "Remove a variable",
        handler: |ctx| {
            crate::commands::unset::unset(ctx)
        },
    },
    CommandDescriptor {
        name: "ifdef",
        signature: "<var>",
        description: "Show inner block if variable exists",
        handler: |ctx| {
            let mut tores = crate::commands::if_n_def::ifdef(ctx)?;
            ctx.result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "ifndef",
        signature: "<var>",
        description: "Show inner block if variable does NOT exist",
        handler: |ctx| {
            let mut tores = crate::commands::if_n_def::ifndef(ctx)?;
            ctx.result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "template",
        signature: "<path> [var# val ...]",
        description: "Include a template with local variables",
        handler: |ctx| {
            let mut tores = crate::commands::template::template(ctx)?;
            ctx.result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "truncate",
        signature: "<text> <length>",
        description: "Truncate string to given length and commute with \"...\"",
        handler: |ctx| {
            let mut tores = crate::commands::truncate::truncate(ctx)?;
            ctx.result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "redir",
        signature: "<url>",
        description: "Create a redirect page",
        handler: |ctx| {
            crate::commands::redir::redir(ctx)
        },
    },
    CommandDescriptor {
        name: "iter_dir",
        signature: "<dir>",
        description: "Iterate directory files; inner block runs per file",
        handler: |ctx| {
            let mut tores = crate::commands::iter_dir::iter_dir(ctx)?;
            ctx.result.append(&mut tores);
            Ok(())
        },
    },
    CommandDescriptor {
        name: "add_document",
        signature: "<docpath> <basename>",
        description: "Add document to base",
        handler: |ctx| {
            crate::commands::add_document::add_document(ctx)
        },
    },
    CommandDescriptor {
        name: "produce_base",
        signature: "<basename>",
        description: "Produce all entries in base",
        handler: |ctx| {
            crate::commands::produce_base::produce_base_cmd(ctx)
        },
    },
    CommandDescriptor {
        name: "load_base",
        signature: "<basename>",
        description: "Load base as JSON string",
        handler: |ctx| {
            crate::commands::load_base::load_base(ctx)
        },
    },
    CommandDescriptor {
        name: "cutbase_extension",
        signature: "<ext> <base>",
        description: "Remove entries not ending with .<ext>",
        handler: |ctx| {
            crate::commands::cutbase_extension::cutbase_extension(ctx)
        },
    },
    CommandDescriptor {
        name: "sortbase_by_key",
        signature: "<key> <base>",
        description: "Sort entries by context key",
        handler: |ctx| {
            crate::commands::sortbase_by_key::sortbase_by_key(ctx)
        },
    },
    CommandDescriptor {
        name: "reverse_base_order",
        signature: "<base>",
        description: "Reverse entry order",
        handler: |ctx| {
            crate::commands::reverse_base_order::reverse_base_order(ctx)
        },
    },
    CommandDescriptor {
        name: "iter_base",
        signature: "<base>",
        description: "Iterate base entries; inner block gets entry data",
        handler: |ctx| {
            crate::commands::iter_base::iter_base(ctx)
        },
    },
    CommandDescriptor {
        name: "gen_doc_from_template",
        signature: "<output> <template>",
        description: "Generate document from template",
        handler: |ctx| {
            crate::commands::gen_doc::gen_doc_from_template(ctx)
        },
    },
    CommandDescriptor {
        name: "ltrim",
        signature: "<string> <count>",
        description: "Left-trim <count> characters",
        handler: |ctx| {
            crate::commands::ltrim::ltrim(ctx)
        },
    },
    CommandDescriptor {
        name: "rtrim",
        signature: "<string> <count>",
        description: "Right-trim <count> characters",
        handler: |ctx| {
            crate::commands::rtrim::rtrim(ctx)
        },
    },
    CommandDescriptor {
        name: "call",
        signature: "...",
        description: "Print evaluated arguments (debug)",
        handler: |ctx| {
            crate::commands::call::call(ctx)
        },
    },
    CommandDescriptor {
        name: "macro",
        signature: "<name> <params>",
        description: "Define macro (debug only)",
        handler: |ctx| {
            crate::commands::macros::create_macro(ctx)?;
            Ok(())
        },
    },
    CommandDescriptor {
        name: "append_data_to_file",
        signature: "<filename> <inner>",
        description: "Append processed inner content to file in output directory",
        handler: |ctx| {
            crate::commands::append_data_to_file::append_data_to_file(ctx)
        },
    },
    CommandDescriptor {
        name: "create_macro",
        signature: "<macro_name> <argnames...> <inner>",
        description: "Create macro of name <macro_name> with <argnames...> and\
                    <inner> content.\n\
                    In macros, the string #![ argname ]!# will be replaced with the\
                    corresponding information when macro_call is used.",
        handler: |ctx| {
            crate::commands::macros::create_macro(ctx)
        },
    },
    CommandDescriptor {
        name: "call_macro",
        signature: "<macro_name> <args...>",
        description: "Use macro of name <macro_name> with <args...>\n\
                    Replacements occur in the order they were defined.",
        handler: |ctx| {
            let mut tores = crate::commands::macros::use_macro(ctx)?;
            ctx.result.append(&mut tores);

            Ok(())
        },
    },
];
