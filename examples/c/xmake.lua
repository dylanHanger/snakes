set_project("my-c-snake")

-- Use debug symbols by default
set_languages("c99")
set_warnings("all", "extra")

-- Global configs
add_rules("mode.debug", "mode.release")
set_defaultmode("debug")

-- Define the C snake target
target("my-c-snake")
    set_kind("binary")
    add_files("main.c")

    -- Windows-only link requirement for ShellExecuteA
    if is_plat("windows") then
        add_links("shell32")
        add_defines("_CRT_SECURE_NO_WARNINGS")
    end