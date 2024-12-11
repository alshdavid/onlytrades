set -e

rm -rf $HOME/.local/terraform
mkdir $HOME/.local/terraform
cd $HOME/.local/terraform

curl "https://releases.hashicorp.com/terraform/1.9.0/terraform_1.9.0_linux_amd64.zip" -o "terraform.zip"  
unzip -qq terraform.zip

export PATH="${HOME}/.local/terraform:${PATH}"
echo "${HOME}/.local/terraform" >> $GITHUB_PATH