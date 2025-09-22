[ -e ./easexx ] && rm ./easexx

EASEXX_DIR=_easexx

git clone https://github.com/KaioFelps/easexx.git $EASEXX_DIR
cd $EASEXX_DIR
cargo build --release
mv target/release/easexx ..
cd ..
rm -rf $EASEXX_DIR
echo "Utilize ./easexx para testar ou compilar."
