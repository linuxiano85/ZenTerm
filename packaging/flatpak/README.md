# Flatpak: build e installazione

## Requisiti
- Flatpak + flatpak-builder
- Runtimes: `org.freedesktop.Platform//23.08`, `org.freedesktop.Sdk//23.08`, `org.freedesktop.Sdk.Extension.rust-stable//23.08`

## Build locale
```bash
sudo apt-get install -y flatpak flatpak-builder
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install -y flathub org.freedesktop.Platform//23.08 org.freedesktop.Sdk//23.08 org.freedesktop.Sdk.Extension.rust-stable//23.08

cd /path/to/ZenTerm
flatpak-builder --user --force-clean --install-deps-from=flathub --repo=repo-dir build-dir packaging/flatpak/zenterm.flatpak.yaml
flatpak build-bundle repo-dir zenterm.flatpak org.zenterm.ZenTerm 23.08
```

## Installazione del bundle
```bash
flatpak install --user zenterm.flatpak
flatpak run org.zenterm.ZenTerm
```

## Note
- Il manifest usa `type: dir` per includere il sorgente corrente (comodo in CI).
- Per Flathub, servirà rifinire icone e metadati; la base è già pronta.
