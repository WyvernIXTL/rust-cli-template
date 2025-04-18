// https://cargo-generate.github.io/cargo-generate/templates/scripting.rhai-extensions.html?highlight=rhai#rhai-extensions

file::delete("README.md");
file::rename("README.template.md", "README.md");

file::delete(".github");
file::rename("github-template", ".github");
