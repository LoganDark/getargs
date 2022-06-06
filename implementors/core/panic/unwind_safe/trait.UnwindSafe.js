(function() {var implementors = {};
implementors["getargs"] = [{"text":"impl&lt;A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"getargs/enum.Arg.html\" title=\"enum getargs::Arg\">Arg</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"getargs/trait.Argument.html\" title=\"trait getargs::Argument\">Argument</a>&gt;::<a class=\"associatedtype\" href=\"getargs/trait.Argument.html#associatedtype.ShortOpt\" title=\"type getargs::Argument::ShortOpt\">ShortOpt</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["getargs::arg::Arg"]},{"text":"impl&lt;A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"getargs/enum.Error.html\" title=\"enum getargs::Error\">Error</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"getargs/trait.Argument.html\" title=\"trait getargs::Argument\">Argument</a>&gt;::<a class=\"associatedtype\" href=\"getargs/trait.Argument.html#associatedtype.ShortOpt\" title=\"type getargs::Argument::ShortOpt\">ShortOpt</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["getargs::error::Error"]},{"text":"impl&lt;'opts, A, I&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"getargs/struct.Positionals.html\" title=\"struct getargs::Positionals\">Positionals</a>&lt;'opts, A, I&gt;","synthetic":true,"types":["getargs::iter::Positionals"]},{"text":"impl&lt;A, I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"getargs/struct.IntoPositionals.html\" title=\"struct getargs::IntoPositionals\">IntoPositionals</a>&lt;A, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["getargs::iter::IntoPositionals"]},{"text":"impl&lt;A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"getargs/enum.Opt.html\" title=\"enum getargs::Opt\">Opt</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"getargs/trait.Argument.html\" title=\"trait getargs::Argument\">Argument</a>&gt;::<a class=\"associatedtype\" href=\"getargs/trait.Argument.html#associatedtype.ShortOpt\" title=\"type getargs::Argument::ShortOpt\">ShortOpt</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["getargs::opt::Opt"]},{"text":"impl&lt;A, I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"getargs/struct.Options.html\" title=\"struct getargs::Options\">Options</a>&lt;A, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"getargs/trait.Argument.html\" title=\"trait getargs::Argument\">Argument</a>&gt;::<a class=\"associatedtype\" href=\"getargs/trait.Argument.html#associatedtype.ShortOpt\" title=\"type getargs::Argument::ShortOpt\">ShortOpt</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["getargs::Options"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()