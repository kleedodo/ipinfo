# 必须遵守

- 添加依赖时不能直接编辑Cargo.toml文件，而是使用cargo add命令
- 如果代码有变动，使用cargo clippy检查代码确保通过编译
- CHANGELOG.md 只能修改 `Unreleased` 章节
- 在完成所有代码编辑后必须使用 cargo fmt 格式化代码

