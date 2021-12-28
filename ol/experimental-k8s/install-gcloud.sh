set -x
mkdir -p ~/gcloud
pushd ~/gcloud
wget https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-sdk-307.0.0-linux-x86_64.tar.gz 
tar xvzf google-cloud-sdk-307.0.0-linux-x86_64.tar.gz
cd google-cloud-sdk
./install.sh
popd
