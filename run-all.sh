for i in $(seq -f "%02g" 1 25)
do
    if [ -d $i ]; then
        cargo run -q --bin advent-of-code-2022-$i
    fi
done