xdg_data_dir :=  "$HOME/.local/share"
plugin_dir := "plugins/JacquesDurand.lapce-php"

build:
    cargo make
    
install-stable: build
    mkdir -p {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/bin
    yes | cp -i bin/lapce-php.wasm {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/bin
    yes | cp -i volt.toml {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/
    rm -rd {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/jdt-language-server-latest || true

install-debug: build
    mkdir -p {{xdg_data_dir}}/lapce-debug/{{plugin_dir}}/bin
    yes | cp -i bin/lapce-php.wasm {{xdg_data_dir}}/lapce-debug/{{plugin_dir}}/bin
    yes | cp -i volt.toml {{xdg_data_dir}}/lapce-debug/{{plugin_dir}}
    rm -rd {{xdg_data_dir}}/lapce-debug/{{plugin_dir}}/jdt-language-server-latest || true