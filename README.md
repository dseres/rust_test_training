# Training material for Rust Testing

This is the training material of a 30 minutes long presentation about testing in Rust programming language. The slides are created in Markdown syntax and can be display with [lookatme](https://lookatme.readthedocs.io/en/latest/index.html).

Install and show presentation:
```bash
git clone https://github.com/dseres/rust_test_training.git
cd rust_test_training
sudo apt install -y libyaml-dev 
python3 -m venv .venv
source ./venv/bin/activate
pip3 install -r requirements.txt
lookatme --theme dark --live slides.md
```