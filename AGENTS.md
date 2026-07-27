# 必须遵守

- 添加依赖时不能直接编辑Cargo.toml文件，而是使用cargo add命令
- 使用cargo clippy检查代码而不是cargo build或者cargo check
- CHANGELOG.md 只能修改 `Unreleased` 章节
- 在完成所有代码编辑后必须使用 cargo fmt 格式化代码
- 不要试图读取或者修改.env文件，对于环境变量的配置应该使用.env.example文件

