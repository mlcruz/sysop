#! /bin/bash

# # 1 conn
# echo 1
# scripts/generic_perf_thread.sh 1 30 stat_generic_1_$1 >bench/stat_generic_threads_1_$1_total_msg.txt
# scripts/generic_perf_task.sh 1 30 stat_generic_1_$1 >bench/stat_generic_tasks_1_$1_total_msg.txt

# echo 2
# scripts/generic_perf_thread.sh 2 30 stat_generic_2_$1 >bench/stat_generic_threads_2_$1_total_msg.txt
# scripts/generic_perf_task.sh 2 30 stat_generic_2_$1 >bench/stat_generic_tasks_2_$1_total_msg.txt

# echo 4
# scripts/generic_perf_thread.sh 4 30 stat_generic_4_$1 >bench/stat_generic_threads_4_$1_total_msg.txt
# scripts/generic_perf_task.sh 4 30 stat_generic_4_$1 >bench/stat_generic_tasks_4_$1_total_msg.txt

# echo 8
# scripts/generic_perf_thread.sh 8 30 stat_generic_8_$1 >bench/stat_generic_threads_8_$1_total_msg.txt
# scripts/generic_perf_task.sh 8 30 stat_generic_8_$1 >bench/stat_generic_tasks_8_$1_total_msg.txt

# echo 16
# scripts/generic_perf_thread.sh 16 30 stat_generic_16_$1 >bench/stat_generic_threads_16_$1_total_msg.txt
# scripts/generic_perf_task.sh 16 30 stat_generic_16_$1 >bench/stat_generic_tasks_16_$1_total_msg.txt

# echo 64
# scripts/generic_perf_thread.sh 64 30 stat_generic_64_$1 >bench/stat_generic_threads_64_$1_total_msg.txt
# scripts/generic_perf_task.sh 64 30 stat_generic_64_$1 >bench/stat_generic_tasks_64_$1_total_msg.txt

# echo 256
# scripts/generic_perf_thread.sh 256 30 stat_generic_256_$1 >bench/stat_generic_threads_256_$1_total_msg.txt
# scripts/generic_perf_task.sh 256 30 stat_generic_256_$1 >bench/stat_generic_tasks_256_$1_total_msg.txt

# echo 512
# scripts/generic_perf_thread.sh 512 30 stat_generic_512_$1 >bench/stat_generic_threads_512_$1_total_msg.txt
# scripts/generic_perf_task.sh 512 30 stat_generic_512_$1 >bench/stat_generic_tasks_512_$1_total_msg.txt

# echo 1024
# scripts/generic_perf_thread.sh 1024 30 stat_generic_1024_$1 >bench/stat_generic_threads_1024_$1_total_msg.txt
# scripts/generic_perf_task.sh 1024 30 stat_generic_1024_$1 >bench/stat_generic_tasks_1024_$1_total_msg.txt

# echo 2048
# scripts/generic_perf_thread.sh 2048 30 stat_generic_2048_$1 >bench/stat_generic_threads_2048_$1_total_msg.txt
# scripts/generic_perf_task.sh 2048 30 stat_generic_2048_$1 >bench/stat_generic_tasks_2048_$1_total_msg.txt

# echo 4096
# scripts/generic_perf_thread.sh 4096 30 stat_generic_4096_$1 >bench/stat_generic_threads_4096_$1_total_msg.txt
# scripts/generic_perf_task.sh 4096 30 stat_generic_4096_$1 >bench/stat_generic_tasks_4096_$1_total_msg.txt

# echo 8192
# scripts/generic_perf_thread.sh 8192 30 stat_generic_8192_$1 >bench/stat_generic_threads_8192_$1_total_msg.txt
# scripts/generic_perf_task.sh 8192 30 stat_generic_8192_$1 >bench/stat_generic_tasks_8192_$1_total_msg.txt

echo 16384
scripts/generic_perf_thread.sh 16384 30 stat_generic_16384_$1 >bench/stat_generic_threads_16384_$1_total_msg.txt
scripts/generic_perf_task.sh 16384 30 stat_generic_16384_$1 >bench/stat_generic_tasks_16384_$1_total_msg.txt

echo 24576
scripts/generic_perf_thread.sh 24576 30 stat_generic_24576_$1 >bench/stat_generic_threads_24576_$1_total_msg.txt
scripts/generic_perf_task.sh 24576 30 stat_generic_24576_$1 >bench/stat_generic_tasks_24576_$1_total_msg.txt
