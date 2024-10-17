echo $(pwd)
source ./.env1
echo "OSS_KEY_ID : $OSS_KEY_ID"
export OSS_KEY_ID=$OSS_KEY_ID
exit 1
# ./oss -i dist -d test/aa11