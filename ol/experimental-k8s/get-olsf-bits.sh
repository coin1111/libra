#set +x
set -e

apps=("db-backup" "db-backup-verify" "db-restore" "diem-node" "tower" "ol" "onboard" "txs")

ol_bin=/ol-bin
web_monitor=/web

for n in ${apps[@]}; do \
  curl  --progress-bar --create-dirs -o "$ol_bin"/$n -L https://github.com/OLSF/libra/releases/latest/download/$n ; \
  echo $n "- downloaded to $ol_bin/" ; \
  chmod 755 "$ol_bin"/$n ;\
done

curl  --progress-bar --create-dirs -o "$web_monitor"/web-monitor.tar.gz -L https://github.com/OLSF/libra/releases/latest/download/web-monitor.tar.gz ; \
echo "web-monitor.tar.gz - downloaded to $web_monitor/" ; \

mkdir -p "$web_monitor"/web-monitor
tar -xf "$web_monitor"/web-monitor.tar.gz --directory "$web_monitor"/web-monitor/
