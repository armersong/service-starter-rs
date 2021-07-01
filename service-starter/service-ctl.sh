#!/bin/sh

BASE_DIR=`dirname $0`
ROOT=$BASE_DIR/..
EXEC_NAME="service-starter"

export BACKTRACE_DEPTH=15
# 打开可能导致大量内存使用。例如 100+MB
#export RUST_BACKTRACE=1

# 开启core dump
ulimit -c unlimited

case "$1" in
    start)
        cd $ROOT && nohup bin/$EXEC_NAME >/dev/null &
        sleep 1
        $0 status
    ;;
    run)
        cd $ROOT && RUST_LOG=info bin/$EXEC_NAME
    ;;
    stop)
        pkill $EXEC_NAME
        sleep 3
        $0 status
    ;;
    status)
        pids=`pgrep $EXEC_NAME`
        if [ -n "$pids" ]; then
            echo "$EXEC_NAME is running $pids"
        else
            echo "$EXEC_NAME is stopped"
        fi
    ;;
    *)
        echo "Usage: $0 {start|run|stop|status}"
        exit 2
    ;;
esac

exit 0
