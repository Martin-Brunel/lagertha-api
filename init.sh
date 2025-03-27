pkcstag=$(openssl rand -hex 32)
pkcsuserpin=$(openssl rand -hex 8)

echo "HSM_TAG=$pkcstag" >> .env
echo "HSM_USER_PIN=$pkcsuserpin" >> .env
echo "HSM_TOKEN_LABEL=Hb_cyber" >> .env

mkdir ssh_keys;
openssl genrsa -out ssh_keys/private.key 2048 && openssl rsa -in ssh_keys/private.key -pubout -out ssh_keys/public.key;