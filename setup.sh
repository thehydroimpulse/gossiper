echo "Initializing environment"
echo "Installing Curl"
if command -v curl >/dev/null 2>&1; then
  echo "Skipping curl installation"
else
  apt-get -y install curl
fi
if command -v rustc >/dev/null 2>&1; then
  echo "Skipping Rust installation..."
else
  echo "Installing Rust"
  curl -s https://static.rust-lang.org/rustup.sh | sudo sh
fi
