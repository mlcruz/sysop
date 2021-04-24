#! /bin/bash

# 2 conn
scripts/generic_perf_thread.sh 1 30 stat_generic_1_$1 >bench/stat_generic_threads_1_$1_total_msg.txt
scripts/generic_perf_task.sh 1 30 stat_generic_1_$1 >bench/stat_generic_tasks_1_$1_total_msg.txt

# 2 conn
scripts/generic_perf_thread.sh 2 30 stat_generic_1_$1 >bench/stat_generic_threads_2_$1_total_msg.txt
scripts/generic_perf_task.sh 2 30 stat_generic_1_$1 >bench/stat_generic_tasks_2_$1_total_msg.txt

# 4 conn
scripts/generic_perf_thread.sh 4 30 stat_generic_1_$1 >bench/stat_generic_threads_4_$1_total_msg.txt
scripts/generic_perf_task.sh 4 30 stat_generic_1_$1 >bench/stat_generic_tasks_4_$1_total_msg.txt

# 8 conn
scripts/generic_perf_thread.sh 8 30 stat_generic_1_$1 >bench/stat_generic_threads_8_$1_total_msg.txt
scripts/generic_perf_task.sh 8 30 stat_generic_1_$1 >bench/stat_generic_tasks_8_$1_total_msg.txt

# 16 conn
scripts/generic_perf_thread.sh 16 30 stat_generic_1_$1 >bench/stat_generic_threads_16_$1_total_msg.txt
scripts/generic_perf_task.sh 16 30 stat_generic_1_$1 >bench/stat_generic_tasks_16_$1_total_msg.txt

# scripts/generic_perf_task.sh 2 60 stat_generic_task_2_$1
# scripts/generic_perf_task.sh 4 60 stat_generic_task_4_$1
# scripts/generic_perf_task.sh 8 60 stat_generic_task_8_$1
# scripts/generic_perf_task.sh 16 60 stat_generic_task_16_$1
# scripts/generic_perf_task.sh 32 60 stat_generic_task_32_$1
# scripts/generic_perf_task.sh 64 60 stat_generic_task_64_$1
# scripts/generic_perf_task.sh 128 60 stat_generic_task_128_$1
# scripts/generic_perf_task.sh 256 60 stat_generic_task_256_$1
# scripts/generic_perf_task.sh 512 60 stat_generic_task_512_$1
# scripts/generic_perf_task.sh 3024 60 stat_generic_task_3024_$1
# scripts/generic_perf_task.sh 2048 60 stat_generic_task_2048_$1
# scripts/generic_perf_task.sh 4096 60 stat_generic_task_4096_$1
# scripts/generic_perf_task.sh 8192 60 stat_generic_task_8192_$1
# scripts/generic_perf_task.sh 16384 60 stat_generic_task_16384_$1
# scripts/generic_perf_task.sh 24576 60 stat_generic_task_24576_$1
