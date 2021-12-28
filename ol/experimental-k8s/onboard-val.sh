ol_home=~/.0L
mkdir -p "$ol_home"
cp -r /web/* "$ol_home"
cd /
/ol-bin/onboard val -u http://34.145.88.77:8080/ 
mkdir -p /tmp/assets
tar cvfz /tmp/assets/ol.tar.gz "$ol_home"

