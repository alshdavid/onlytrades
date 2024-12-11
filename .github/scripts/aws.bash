set -e

rm -rf $HOME/.local/aws
mkdir $HOME/.local/aws
cd $HOME/.local/aws

curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"  
unzip -qq awscliv2.zip
./aws/install --bin-dir $HOME/.local/aws/bin --install-dir $HOME/.local/aws/cli --update

export PATH="${HOME}/.local/aws/bin:${PATH}"
echo "${HOME}/.local/aws/bin" >> $GITHUB_PATH