file::delete("README.md");
file::rename("README.template.md", "README.md");

file::delete(".github");
file::rename("github-template", ".github");
