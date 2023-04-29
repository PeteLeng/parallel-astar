usage() {
    echo "$0 usage:" && grep ".)\ #" $0;
    exit 1;
}
[ $# -eq 0 ] && usage

commit=false
while getopts ":cdh" opt; do
    case "$opt" in
	c) # Commit push to cycle
	    commit=true;;
	d) # Dry run
	;;
	h | *) # Dispaly help
	    usage
    esac
done

# Push to cycle machine
SRC_DIR=(~/prog/rust/astar)
DST_DIR=yleng2@cycle1.csug.rochester.edu:/u/yleng2/term/
EXL_FLAGS=('*~' '*#' 'push_to_cycle.sh' target/ .git/ plots/)

exl_args=()
for f in "${EXL_FLAGS[@]}";
do exl_args+=(--exclude="$f");
done

if "$commit"; then
    echo "Commit push to cycle!"
    rsync -av --prune-empty-dirs "${exl_args[@]}" "${SRC_DIR[@]}" "$DST_DIR"
else
    echo "Dry run pull."
    rsync -av --prune-empty-dirs --dry-run "${exl_args[@]}" "${SRC_DIR[@]}" "$DST_DIR"
fi
