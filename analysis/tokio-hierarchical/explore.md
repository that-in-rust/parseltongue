# Detailed Architecture - Levels 2-3 (1,000ft view)

This view shows the detailed module structure and key relationships.
*⬅️ Back to: [Overview](index.md) | 🗂️ Full Data: [JSON Export](data/full_isg.json)*

```mermaid
flowchart TD

    %% Level 1: 4 directories at depth 0
    L1_Dbenches_N0["🔧 contention_bounded_full_recv_many<br/><i>(Function)<br/>sync_mpsc.rs</i>"]
    L1_Dbenches_N1["🔧 copy_chunk_to_slow_hdd<br/><i>(Function)<br/>copy.rs</i>"]
    L1_Dbenches_N2["🔧 contended_concurrent_single<br/><i>(Function)<br/>sync_semaphore.rs</i>"]
    L1_Dbenches_N3["🔧 uncontented_unbounded<br/><i>(Function)<br/>sync_mpsc.rs</i>"]
    L1_Dbenches_N4["🔧 request_reply_multi_threaded<br/><i>(Function)<br/>sync_mpsc_oneshot.rs</i>"]
    L1_Dbenches_N5["🔧 rt_multi_spawn_many_local<br/><i>(Function)<br/>rt_multi_threaded.rs</i>"]
    L1_Dbenches_N6["🔧 build_run_time<br/><i>(Function)<br/>time_timeout.rs</i>"]
    L1_Dbenches_N7["🔧 spawn_sleep_job<br/><i>(Function)<br/>time_timeout.rs</i>"]
    L1_Dbenches_N8["🔧 rt_curr_spawn_many_local<br/><i>(Function)<br/>rt_current_thread.rs</i>"]
    L1_Dbenches_N9["🔧 read_concurrent_uncontended_multi<br/><i>(Function)<br/>sync_rwlock.rs</i>"]
    L1_Dexamples_N0["🔧 pong<br/><i>(Function)<br/>udp-codec.rs</i>"]
    L1_Dexamples_N1["📦 Peer<br/><i>(Struct)<br/>chat.rs</i>"]
    L1_Dexamples_N2["🔧 main<br/><i>(Function)<br/>udp-codec.rs</i>"]
    L1_Dexamples_N3["🔧 main<br/><i>(Function)<br/>dump.rs</i>"]
    L1_Dexamples_N4["📦 Database<br/><i>(Struct)<br/>tinydb.rs</i>"]
    L1_Dexamples_N5["🔧 send<br/><i>(Function)<br/>connect-udp.rs</i>"]
    L1_Dexamples_N6["🔧 process<br/><i>(Function)<br/>tinyhttp.rs</i>"]
    L1_Dexamples_N7["🔧 main<br/><i>(Function)<br/>print_each_packet.rs</i>"]
    L1_Dexamples_N8["🔧 main<br/><i>(Function)<br/>proxy.rs</i>"]
    L1_Dexamples_N9["🔧 main<br/><i>(Function)<br/>udp-client.rs</i>"]
    L1_Dsrc_N0["🎯 Greeter<br/><i>(Trait)<br/>lib.rs</i>"]
    L1_Dsrc_N1["🔧 main<br/><i>(Function)<br/>main.rs</i>"]
    L1_Dsrc_N2["📦 Person<br/><i>(Struct)<br/>lib.rs</i>"]
    L1_Dsrc_N3["🔧 hello<br/><i>(Function)<br/>lib.rs</i>"]
    L1_Dsrc_N4["🔧 create_user<br/><i>(Function)<br/>lib.rs</i>"]
    L1_Dsrc_N5["🎯 Display<br/><i>(Trait)<br/>lib.rs</i>"]
    L1_Dsrc_N6["📦 User<br/><i>(Struct)<br/>lib.rs</i>"]
    L1_D._N0["🔧 main<br/><i>(Function)<br/>example.rs</i>"]
    L1_D._N1["🔧 test<br/><i>(Function)<br/>test.rs</i>"]

    %% Level 2: 26 directories at depth 2
    L2_Dtests_build_tests_fail_N0["🔧 test_has_second_test_attr_v1<br/><i>(Function)<br/>macros_invalid_input.rs</i>"]
    L2_Dtests_build_tests_fail_N1["🔧 missing_semicolon_or_return_type<br/><i>(Function)<br/>macros_type_mismatch.rs</i>"]
    L2_Dtests_build_tests_fail_N2["🔧 test_has_second_test_attr<br/><i>(Function)<br/>macros_invalid_input.rs</i>"]
    L2_Dtests_build_tests_fail_N3["🔧 test_is_not_async<br/><i>(Function)<br/>macros_invalid_input.rs</i>"]
    L2_Dtests_build_tests_fail_N4["🔧 question_mark_operator_with_invalid_option<br/><i>(Function)<br/>macros_type_mismatch.rs</i>"]
    L2_Dtests_build_tests_fail_N5["🔧 test_unexpected_attr<br/><i>(Function)<br/>macros_invalid_input.rs</i>"]
    L2_Dtests_build_tests_fail_N6["🔧 issue_4635<br/><i>(Function)<br/>macros_type_mismatch.rs</i>"]
    L2_Dtests_build_tests_fail_N7["🔧 test_multi_thread_with_unhandled_panic<br/><i>(Function)<br/>macros_invalid_input.rs</i>"]
    L2_Dtests_build_tests_fail_N8["🔧 test_attr_has_args<br/><i>(Function)<br/>macros_invalid_input.rs</i>"]
    L2_Dtests_build_tests_fail_N9["🔧 f<br/><i>(Function)<br/>macros_dead_code.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N0["📦 CtrlBreakStream<br/><i>(Struct)<br/>signal_windows.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N1["📦 ReceiverStream<br/><i>(Struct)<br/>mpsc_bounded.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N2["📦 ReadDirStream<br/><i>(Struct)<br/>read_dir.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N3["📦 TcpListenerStream<br/><i>(Struct)<br/>tcp_listener.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N4["📦 BroadcastStream<br/><i>(Struct)<br/>broadcast.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N5["📦 WatchStream<br/><i>(Struct)<br/>watch.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N6["📦 SignalStream<br/><i>(Struct)<br/>signal_unix.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N7["📦 UnixListenerStream<br/><i>(Struct)<br/>unix_listener.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N8["🔧 make_future<br/><i>(Function)<br/>broadcast.rs</i>"]
    L2_Dtokio_stream_src_wrappers_N9["🔧 make_future<br/><i>(Function)<br/>watch.rs</i>"]
    L2_Dtokio_src_task_N0["📦 JoinSet<br/><i>(Struct)<br/>join_set.rs</i>"]
    L2_Dtokio_src_task_N1["📦 LocalDataEnterGuard<br/><i>(Struct)<br/>local.rs</i>"]
    L2_Dtokio_src_task_N2["📦 LocalKey<br/><i>(Struct)<br/>task_local.rs</i>"]
    L2_Dtokio_src_task_N3["📦 LocalEnterGuard<br/><i>(Struct)<br/>local.rs</i>"]
    L2_Dtokio_src_task_N4["📦 Shared<br/><i>(Struct)<br/>local.rs</i>"]
    L2_Dtokio_src_task_N5["📦 AccessError<br/><i>(Struct)<br/>task_local.rs</i>"]
    L2_Dtokio_src_task_N6["📦 LocalState<br/><i>(Struct)<br/>local.rs</i>"]
    L2_Dtokio_src_task_N7["🔧 yield_now<br/><i>(Function)<br/>yield_now.rs</i>"]
    L2_Dtokio_src_task_N8["📦 LocalData<br/><i>(Struct)<br/>local.rs</i>"]
    L2_Dtokio_src_task_N9["📦 Context<br/><i>(Struct)<br/>local.rs</i>"]
    L2_Dtokio_src_util_N0["🔧 move_to_new_list<br/><i>(Function)<br/>idle_notified_set.rs</i>"]
    L2_Dtokio_src_util_N1["📦 AtomicCell<br/><i>(Struct)<br/>atomic_cell.rs</i>"]
    L2_Dtokio_src_util_N2["🔧 from_raw<br/><i>(Function)<br/>atomic_cell.rs</i>"]
    L2_Dtokio_src_util_N3["📦 IdleNotifiedSet<br/><i>(Struct)<br/>idle_notified_set.rs</i>"]
    L2_Dtokio_src_util_N4["📦 LinkedList<br/><i>(Struct)<br/>linked_list.rs</i>"]
    L2_Dtokio_src_util_N5["📦 WakeList<br/><i>(Struct)<br/>wake_list.rs</i>"]
    L2_Dtokio_src_util_N6["🔧 nonstatic_typeid<br/><i>(Function)<br/>typeid.rs</i>"]
    L2_Dtokio_src_util_N7["🎯 Wake<br/><i>(Trait)<br/>wake.rs</i>"]
    L2_Dtokio_src_util_N8["🔧 wake_by_ref_arc_raw<br/><i>(Function)<br/>wake.rs</i>"]
    L2_Dtokio_src_util_N9["📦 SyncWrapper<br/><i>(Struct)<br/>sync_wrapper.rs</i>"]
    L2_Dtokio_src_io_N0["📦 SplitByUtf8BoundaryIfWindows<br/><i>(Struct)<br/>stdio_common.rs</i>"]
    L2_Dtokio_src_io_N1["📦 Ready<br/><i>(Struct)<br/>ready.rs</i>"]
    L2_Dtokio_src_io_N2["📦 ReadBuf<br/><i>(Struct)<br/>read_buf.rs</i>"]
    L2_Dtokio_src_io_N3["🔧 slice_to_uninit_mut<br/><i>(Function)<br/>read_buf.rs</i>"]
    L2_Dtokio_src_io_N4["📦 Blocking<br/><i>(Struct)<br/>blocking.rs</i>"]
    L2_Dtokio_src_io_N5["🔧 join<br/><i>(Function)<br/>join.rs</i>"]
    L2_Dtokio_src_io_N6["🎯 AsyncWrite<br/><i>(Trait)<br/>async_write.rs</i>"]
    L2_Dtokio_src_io_N7["🔧 slice_assume_init<br/><i>(Function)<br/>read_buf.rs</i>"]
    L2_Dtokio_src_io_N8["🔧 seek<br/><i>(Function)<br/>seek.rs</i>"]
    L2_Dtokio_src_io_N9["🎯 AsyncSeek<br/><i>(Trait)<br/>async_seek.rs</i>"]
    L2_Dtokio_src_sync_N0["📦 Waitlist<br/><i>(Struct)<br/>batch_semaphore.rs</i>"]
    L2_Dtokio_src_sync_N1["📦 Barrier<br/><i>(Struct)<br/>barrier.rs</i>"]
    L2_Dtokio_src_sync_N2["📦 NotifiedProject<br/><i>(Struct)<br/>notify.rs</i>"]
    L2_Dtokio_src_sync_N3["📦 OwnedMappedMutexGuardInner<br/><i>(Struct)<br/>mutex.rs</i>"]
    L2_Dtokio_src_sync_N4["🔧 is_unpin<br/><i>(Function)<br/>notify.rs</i>"]
    L2_Dtokio_src_sync_N5["🔧 get_state<br/><i>(Function)<br/>notify.rs</i>"]
    L2_Dtokio_src_sync_N6["📦 SetOnce<br/><i>(Struct)<br/>set_once.rs</i>"]
    L2_Dtokio_src_sync_N7["📦 State<br/><i>(Struct)<br/>oneshot.rs</i>"]
    L2_Dtokio_src_sync_N8["📦 AcquireError<br/><i>(Struct)<br/>batch_semaphore.rs</i>"]
    L2_Dtokio_src_sync_N9["📦 MutexGuard<br/><i>(Struct)<br/>mutex.rs</i>"]
    L2_Dtokio_src_fs_N0["📦 ReadDir<br/><i>(Struct)<br/>read_dir.rs</i>"]
    L2_Dtokio_src_fs_N1["📦 DirBuilder<br/><i>(Struct)<br/>dir_builder.rs</i>"]
    L2_Dtokio_src_fs_N2["🔧 canonicalize<br/><i>(Function)<br/>canonicalize.rs</i>"]
    L2_Dtokio_src_fs_N3["🔧 spawn_mandatory_blocking<br/><i>(Function)<br/>mocks.rs</i>"]
    L2_Dtokio_src_fs_N4["📦 DirEntry<br/><i>(Struct)<br/>read_dir.rs</i>"]
    L2_Dtokio_src_fs_N5["🔧 read_link<br/><i>(Function)<br/>read_link.rs</i>"]
    L2_Dtokio_src_fs_N6["🔧 write_uring<br/><i>(Function)<br/>write.rs</i>"]
    L2_Dtokio_src_fs_N7["🔧 create_dir<br/><i>(Function)<br/>create_dir.rs</i>"]
    L2_Dtokio_src_fs_N8["🔧 symlink<br/><i>(Function)<br/>symlink.rs</i>"]
    L2_Dtokio_src_fs_N9["🔧 remove_file<br/><i>(Function)<br/>remove_file.rs</i>"]
    L2_Dtokio_stream_src_stream_ext_N0["🎯 FromStream<br/><i>(Trait)<br/>collect.rs</i>"]
    L2_Dtokio_stream_src_stream_ext_N1["🔧 is_zero<br/><i>(Function)<br/>throttle.rs</i>"]
    L2_Dtokio_stream_src_stream_ext_N2["🔧 poll_next<br/><i>(Function)<br/>merge.rs</i>"]
    L2_Dtokio_stream_src_stream_ext_N3["📦 Elapsed<br/><i>(Struct)<br/>timeout.rs</i>"]
    L2_Dtokio_stream_src_stream_ext_N4["🔧 throttle<br/><i>(Function)<br/>throttle.rs</i>"]
    L2_Dtokio_src_signal_N0["🔧 action<br/><i>(Function)<br/>unix.rs</i>"]
    L2_Dtokio_src_signal_N1["📦 CtrlShutdown<br/><i>(Struct)<br/>windows.rs</i>"]
    L2_Dtokio_src_signal_N2["🔧 ctrl_close<br/><i>(Function)<br/>windows.rs</i>"]
    L2_Dtokio_src_signal_N3["📦 CtrlC<br/><i>(Struct)<br/>windows.rs</i>"]
    L2_Dtokio_src_signal_N4["🔧 signal<br/><i>(Function)<br/>unix.rs</i>"]
    L2_Dtokio_src_signal_N5["📦 Globals<br/><i>(Struct)<br/>registry.rs</i>"]
    L2_Dtokio_src_signal_N6["📦 CtrlBreak<br/><i>(Struct)<br/>windows.rs</i>"]
    L2_Dtokio_src_signal_N7["🔧 ctrl_break<br/><i>(Function)<br/>windows.rs</i>"]
    L2_Dtokio_src_signal_N8["🔧 globals_init<br/><i>(Function)<br/>registry.rs</i>"]
    L2_Dtokio_src_signal_N9["📦 CtrlClose<br/><i>(Struct)<br/>windows.rs</i>"]
    L2_Dtokio_util_src_util_N0["🔧 maybedangling_runs_drop<br/><i>(Function)<br/>maybe_dangling.rs</i>"]
    L2_Dtokio_util_src_util_N1["🔧 poll_write_buf<br/><i>(Function)<br/>poll_buf.rs</i>"]
    L2_Dtokio_util_src_util_N2["🔧 poll_read_buf<br/><i>(Function)<br/>poll_buf.rs</i>"]
    L2_Dtokio_util_src_util_N3["📦 MaybeDangling<br/><i>(Struct)<br/>maybe_dangling.rs</i>"]
    L2_Dtokio_util_src_time_N0["📦 Stack<br/><i>(Struct)<br/>delay_queue.rs</i>"]
    L2_Dtokio_util_src_time_N1["🔧 ms<br/><i>(Function)<br/>mod.rs</i>"]
    L2_Dtokio_util_src_time_N2["📦 SlabStorage<br/><i>(Struct)<br/>delay_queue.rs</i>"]
    L2_Dtokio_util_src_time_N3["📦 Expired<br/><i>(Struct)<br/>delay_queue.rs</i>"]
    L2_Dtokio_util_src_time_N4["📦 Data<br/><i>(Struct)<br/>delay_queue.rs</i>"]
    L2_Dtokio_util_src_time_N5["📦 KeyInternal<br/><i>(Struct)<br/>delay_queue.rs</i>"]
    L2_Dtokio_util_src_time_N6["📦 DelayQueue<br/><i>(Struct)<br/>delay_queue.rs</i>"]
    L2_Dtokio_util_src_time_N7["📦 Key<br/><i>(Struct)<br/>delay_queue.rs</i>"]
    L2_Dtokio_util_src_net_N0["🎯 Listener<br/><i>(Trait)<br/>mod.rs</i>"]
    L2_Dtokio_util_src_net_N1["📦 ListenerAcceptFut<br/><i>(Struct)<br/>mod.rs</i>"]
    L2_Dtokio_tests_support_N0["📦 BoundedStream<br/><i>(Struct)<br/>mpsc_stream.rs</i>"]
    L2_Dtokio_tests_support_N1["🔧 send_signal<br/><i>(Function)<br/>signal.rs</i>"]
    L2_Dtokio_tests_support_N2["📦 LeakedBuffers<br/><i>(Struct)<br/>leaked_buffers.rs</i>"]
    L2_Dtokio_tests_support_N3["🔧 test_panic<br/><i>(Function)<br/>panic.rs</i>"]
    L2_Dtokio_tests_support_N4["📦 UnboundedStream<br/><i>(Struct)<br/>mpsc_stream.rs</i>"]
    L2_Dtokio_tests_support_N5["🔧 channel_stream<br/><i>(Function)<br/>mpsc_stream.rs</i>"]
    L2_Dtokio_tests_support_N6["🔧 unbounded_channel_stream<br/><i>(Function)<br/>mpsc_stream.rs</i>"]
    L2_Dtokio_tests_support_N7["📦 IoBufs<br/><i>(Struct)<br/>io_vec.rs</i>"]
    L2_Dtokio_src_time_N0["🔧 sleep_until<br/><i>(Function)<br/>sleep.rs</i>"]
    L2_Dtokio_src_time_N1["🔧 interval<br/><i>(Function)<br/>interval.rs</i>"]
    L2_Dtokio_src_time_N2["🔧 timeout<br/><i>(Function)<br/>timeout.rs</i>"]
    L2_Dtokio_src_time_N3["📦 Instant<br/><i>(Struct)<br/>instant.rs</i>"]
    L2_Dtokio_src_time_N4["🔧 internal_interval_at<br/><i>(Function)<br/>interval.rs</i>"]
    L2_Dtokio_src_time_N5["🔧 interval_at<br/><i>(Function)<br/>interval.rs</i>"]
    L2_Dtokio_src_time_N6["🔧 sleep<br/><i>(Function)<br/>sleep.rs</i>"]
    L2_Dtokio_src_time_N7["🔧 timeout_at<br/><i>(Function)<br/>timeout.rs</i>"]
    L2_Dtokio_src_time_N8["📦 Interval<br/><i>(Struct)<br/>interval.rs</i>"]
    L2_Dtokio_src_time_N9["🔧 poll_delay<br/><i>(Function)<br/>timeout.rs</i>"]
    L2_Dtokio_util_src_task_N0["📦 LocalPoolHandle<br/><i>(Struct)<br/>spawn_pinned.rs</i>"]
    L2_Dtokio_util_src_task_N1["📦 TaskTracker<br/><i>(Struct)<br/>task_tracker.rs</i>"]
    L2_Dtokio_util_src_task_N2["📦 LocalWorkerHandle<br/><i>(Struct)<br/>spawn_pinned.rs</i>"]
    L2_Dtokio_util_src_task_N3["📦 AbortOnDropHandle<br/><i>(Struct)<br/>abort_on_drop.rs</i>"]
    L2_Dtokio_util_src_task_N4["🔧 debug_inner<br/><i>(Function)<br/>task_tracker.rs</i>"]
    L2_Dtokio_util_src_task_N5["📦 AbortGuard<br/><i>(Struct)<br/>spawn_pinned.rs</i>"]
    L2_Dtokio_util_src_task_N6["📦 JobCountGuard<br/><i>(Struct)<br/>spawn_pinned.rs</i>"]
    L2_Dtokio_util_src_task_N7["📦 JoinMapKeys<br/><i>(Struct)<br/>join_map.rs</i>"]
    L2_Dtokio_util_src_task_N8["📦 TaskTrackerToken<br/><i>(Struct)<br/>task_tracker.rs</i>"]
    L2_Dtokio_util_src_task_N9["📦 JoinQueue<br/><i>(Struct)<br/>join_queue.rs</i>"]
    L2_Dtokio_src_process_N0["📦 Waiting<br/><i>(Struct)<br/>windows.rs</i>"]
    L2_Dtokio_src_process_N1["🔧 duplicate_handle<br/><i>(Function)<br/>windows.rs</i>"]
    L2_Dtokio_src_process_N2["🔧 build_child<br/><i>(Function)<br/>windows.rs</i>"]
    L2_Dtokio_src_process_N3["🔧 convert_to_file<br/><i>(Function)<br/>windows.rs</i>"]
    L2_Dtokio_src_process_N4["📦 ArcFile<br/><i>(Struct)<br/>windows.rs</i>"]
    L2_Dtokio_src_process_N5["🔧 callback<br/><i>(Function)<br/>windows.rs</i>"]
    L2_Dtokio_src_process_N6["🔧 convert_to_stdio<br/><i>(Function)<br/>windows.rs</i>"]
    L2_Dtokio_src_process_N7["📦 Child<br/><i>(Struct)<br/>windows.rs</i>"]
    L2_Dtokio_src_process_N8["🔧 stdio<br/><i>(Function)<br/>windows.rs</i>"]
    L2_Dtokio_src_process_N9["🎯 Kill<br/><i>(Trait)<br/>kill.rs</i>"]
    L2_Dtokio_src_runtime_N0["📦 BacktraceFrame<br/><i>(Struct)<br/>dump.rs</i>"]
    L2_Dtokio_src_runtime_N1["📦 Tasks<br/><i>(Struct)<br/>dump.rs</i>"]
    L2_Dtokio_src_runtime_N2["📦 TaskMeta<br/><i>(Struct)<br/>task_hooks.rs</i>"]
    L2_Dtokio_src_runtime_N3["📦 Dump<br/><i>(Struct)<br/>dump.rs</i>"]
    L2_Dtokio_src_runtime_N4["📦 UnparkThread<br/><i>(Struct)<br/>park.rs</i>"]
    L2_Dtokio_src_runtime_N5["📦 Address<br/><i>(Struct)<br/>dump.rs</i>"]
    L2_Dtokio_src_runtime_N6["📦 EnterGuard<br/><i>(Struct)<br/>handle.rs</i>"]
    L2_Dtokio_src_runtime_N7["📦 Backtrace<br/><i>(Struct)<br/>dump.rs</i>"]
    L2_Dtokio_src_runtime_N8["🔧 exhausted<br/><i>(Function)<br/>thread_id.rs</i>"]
    L2_Dtokio_src_runtime_N9["📦 ThreadId<br/><i>(Struct)<br/>thread_id.rs</i>"]
    L2_Dtokio_src_macros_N0["📦 SelectNormal<br/><i>(Struct)<br/>join.rs</i>"]
    L2_Dtokio_src_macros_N1["📦 Rotator<br/><i>(Struct)<br/>join.rs</i>"]
    L2_Dtokio_src_macros_N2["📦 BiasedRotator<br/><i>(Struct)<br/>join.rs</i>"]
    L2_Dtokio_src_macros_N3["📦 SelectBiased<br/><i>(Struct)<br/>join.rs</i>"]
    L2_Dtokio_src_macros_N4["🎯 RotatorSelect<br/><i>(Trait)<br/>join.rs</i>"]
    L2_Dtokio_util_src_io_N0["📦 StreamReaderProject<br/><i>(Struct)<br/>stream_reader.rs</i>"]
    L2_Dtokio_util_src_io_N1["📦 SyncIoBridge<br/><i>(Struct)<br/>sync_bridge.rs</i>"]
    L2_Dtokio_util_src_io_N2["🔧 read_exact_arc<br/><i>(Function)<br/>read_arc.rs</i>"]
    L2_Dtokio_util_src_io_N3["🔧 read_buf<br/><i>(Function)<br/>read_buf.rs</i>"]
    L2_Dtokio_util_src_io_N4["📦 StreamReader<br/><i>(Struct)<br/>stream_reader.rs</i>"]
    L2_Dtokio_util_src_codec_N0["🔧 without_carriage_return<br/><i>(Function)<br/>lines_codec.rs</i>"]
    L2_Dtokio_util_src_codec_N1["📦 Builder<br/><i>(Struct)<br/>length_delimited.rs</i>"]
    L2_Dtokio_util_src_codec_N2["🔧 utf8<br/><i>(Function)<br/>lines_codec.rs</i>"]
    L2_Dtokio_util_src_codec_N3["📦 AnyDelimiterCodec<br/><i>(Struct)<br/>any_delimiter_codec.rs</i>"]
    L2_Dtokio_util_src_codec_N4["📦 RWFrames<br/><i>(Struct)<br/>framed_impl.rs</i>"]
    L2_Dtokio_util_src_codec_N5["📦 ReadFrame<br/><i>(Struct)<br/>framed_impl.rs</i>"]
    L2_Dtokio_util_src_codec_N6["📦 LengthDelimitedCodec<br/><i>(Struct)<br/>length_delimited.rs</i>"]
    L2_Dtokio_util_src_codec_N7["📦 LengthDelimitedCodecError<br/><i>(Struct)<br/>length_delimited.rs</i>"]
    L2_Dtokio_util_src_codec_N8["📦 WriteFrame<br/><i>(Struct)<br/>framed_impl.rs</i>"]
    L2_Dtokio_util_src_codec_N9["🎯 Encoder<br/><i>(Trait)<br/>encoder.rs</i>"]
    L2_Dtokio_util_src_sync_N0["📦 ReusableBoxFuture<br/><i>(Struct)<br/>reusable_box.rs</i>"]
    L2_Dtokio_util_src_sync_N1["📦 CallOnDrop<br/><i>(Struct)<br/>reusable_box.rs</i>"]
    L2_Dtokio_util_src_sync_N2["📦 PollSenderFuture<br/><i>(Struct)<br/>mpsc.rs</i>"]
    L2_Dtokio_util_src_sync_N3["🔧 reuse_pin_box<br/><i>(Function)<br/>reusable_box.rs</i>"]

    %% Level 3: 10 directories at depth 4

    %% Directory groupings
    subgraph SubL1["📁 benches (Level 1)"]
        L1_Dbenches_N0
        L1_Dbenches_N1
        L1_Dbenches_N2
        L1_Dbenches_N3
        L1_Dbenches_N4
        L1_Dbenches_N5
        L1_Dbenches_N6
        L1_Dbenches_N7
        L1_Dbenches_N8
        L1_Dbenches_N9
    end
    subgraph SubL1["📁 examples (Level 1)"]
        L1_Dexamples_N0
        L1_Dexamples_N1
        L1_Dexamples_N2
        L1_Dexamples_N3
        L1_Dexamples_N4
        L1_Dexamples_N5
        L1_Dexamples_N6
        L1_Dexamples_N7
        L1_Dexamples_N8
        L1_Dexamples_N9
    end
    subgraph SubL1["📁 src (Level 1)"]
        L1_Dsrc_N0
        L1_Dsrc_N1
        L1_Dsrc_N2
        L1_Dsrc_N3
        L1_Dsrc_N4
        L1_Dsrc_N5
        L1_Dsrc_N6
    end
    subgraph SubL1["📁 . (Level 1)"]
        L1_D._N0
        L1_D._N1
    end
    subgraph SubL2["📁 tests-build/tests/fail (Level 2)"]
        L2_Dtests_build_tests_fail_N0
        L2_Dtests_build_tests_fail_N1
        L2_Dtests_build_tests_fail_N2
        L2_Dtests_build_tests_fail_N3
        L2_Dtests_build_tests_fail_N4
        L2_Dtests_build_tests_fail_N5
        L2_Dtests_build_tests_fail_N6
        L2_Dtests_build_tests_fail_N7
        L2_Dtests_build_tests_fail_N8
        L2_Dtests_build_tests_fail_N9
    end
    subgraph SubL2["📁 tokio-stream/src/wrappers (Level 2)"]
        L2_Dtokio_stream_src_wrappers_N0
        L2_Dtokio_stream_src_wrappers_N1
        L2_Dtokio_stream_src_wrappers_N2
        L2_Dtokio_stream_src_wrappers_N3
        L2_Dtokio_stream_src_wrappers_N4
        L2_Dtokio_stream_src_wrappers_N5
        L2_Dtokio_stream_src_wrappers_N6
        L2_Dtokio_stream_src_wrappers_N7
        L2_Dtokio_stream_src_wrappers_N8
        L2_Dtokio_stream_src_wrappers_N9
    end
    subgraph SubL2["📁 tokio/src/task (Level 2)"]
        L2_Dtokio_src_task_N0
        L2_Dtokio_src_task_N1
        L2_Dtokio_src_task_N2
        L2_Dtokio_src_task_N3
        L2_Dtokio_src_task_N4
        L2_Dtokio_src_task_N5
        L2_Dtokio_src_task_N6
        L2_Dtokio_src_task_N7
        L2_Dtokio_src_task_N8
        L2_Dtokio_src_task_N9
    end
    subgraph SubL2["📁 tokio/src/util (Level 2)"]
        L2_Dtokio_src_util_N0
        L2_Dtokio_src_util_N1
        L2_Dtokio_src_util_N2
        L2_Dtokio_src_util_N3
        L2_Dtokio_src_util_N4
        L2_Dtokio_src_util_N5
        L2_Dtokio_src_util_N6
        L2_Dtokio_src_util_N7
        L2_Dtokio_src_util_N8
        L2_Dtokio_src_util_N9
    end
    subgraph SubL2["📁 tokio/src/io (Level 2)"]
        L2_Dtokio_src_io_N0
        L2_Dtokio_src_io_N1
        L2_Dtokio_src_io_N2
        L2_Dtokio_src_io_N3
        L2_Dtokio_src_io_N4
        L2_Dtokio_src_io_N5
        L2_Dtokio_src_io_N6
        L2_Dtokio_src_io_N7
        L2_Dtokio_src_io_N8
        L2_Dtokio_src_io_N9
    end
    subgraph SubL2["📁 tokio/src/sync (Level 2)"]
        L2_Dtokio_src_sync_N0
        L2_Dtokio_src_sync_N1
        L2_Dtokio_src_sync_N2
        L2_Dtokio_src_sync_N3
        L2_Dtokio_src_sync_N4
        L2_Dtokio_src_sync_N5
        L2_Dtokio_src_sync_N6
        L2_Dtokio_src_sync_N7
        L2_Dtokio_src_sync_N8
        L2_Dtokio_src_sync_N9
    end
    subgraph SubL2["📁 tokio/src/fs (Level 2)"]
        L2_Dtokio_src_fs_N0
        L2_Dtokio_src_fs_N1
        L2_Dtokio_src_fs_N2
        L2_Dtokio_src_fs_N3
        L2_Dtokio_src_fs_N4
        L2_Dtokio_src_fs_N5
        L2_Dtokio_src_fs_N6
        L2_Dtokio_src_fs_N7
        L2_Dtokio_src_fs_N8
        L2_Dtokio_src_fs_N9
    end
    subgraph SubL2["📁 tokio-stream/src/stream_ext (Level 2)"]
        L2_Dtokio_stream_src_stream_ext_N0
        L2_Dtokio_stream_src_stream_ext_N1
        L2_Dtokio_stream_src_stream_ext_N2
        L2_Dtokio_stream_src_stream_ext_N3
        L2_Dtokio_stream_src_stream_ext_N4
    end
    subgraph SubL2["📁 tokio/src/signal (Level 2)"]
        L2_Dtokio_src_signal_N0
        L2_Dtokio_src_signal_N1
        L2_Dtokio_src_signal_N2
        L2_Dtokio_src_signal_N3
        L2_Dtokio_src_signal_N4
        L2_Dtokio_src_signal_N5
        L2_Dtokio_src_signal_N6
        L2_Dtokio_src_signal_N7
        L2_Dtokio_src_signal_N8
        L2_Dtokio_src_signal_N9
    end
    subgraph SubL2["📁 tokio-util/src/util (Level 2)"]
        L2_Dtokio_util_src_util_N0
        L2_Dtokio_util_src_util_N1
        L2_Dtokio_util_src_util_N2
        L2_Dtokio_util_src_util_N3
    end
    subgraph SubL2["📁 tokio-util/src/time (Level 2)"]
        L2_Dtokio_util_src_time_N0
        L2_Dtokio_util_src_time_N1
        L2_Dtokio_util_src_time_N2
        L2_Dtokio_util_src_time_N3
        L2_Dtokio_util_src_time_N4
        L2_Dtokio_util_src_time_N5
        L2_Dtokio_util_src_time_N6
        L2_Dtokio_util_src_time_N7
    end
    subgraph SubL2["📁 tokio-util/src/net (Level 2)"]
        L2_Dtokio_util_src_net_N0
        L2_Dtokio_util_src_net_N1
    end
    subgraph SubL2["📁 tokio/tests/support (Level 2)"]
        L2_Dtokio_tests_support_N0
        L2_Dtokio_tests_support_N1
        L2_Dtokio_tests_support_N2
        L2_Dtokio_tests_support_N3
        L2_Dtokio_tests_support_N4
        L2_Dtokio_tests_support_N5
        L2_Dtokio_tests_support_N6
        L2_Dtokio_tests_support_N7
    end
    subgraph SubL2["📁 tokio/src/time (Level 2)"]
        L2_Dtokio_src_time_N0
        L2_Dtokio_src_time_N1
        L2_Dtokio_src_time_N2
        L2_Dtokio_src_time_N3
        L2_Dtokio_src_time_N4
        L2_Dtokio_src_time_N5
        L2_Dtokio_src_time_N6
        L2_Dtokio_src_time_N7
        L2_Dtokio_src_time_N8
        L2_Dtokio_src_time_N9
    end
    subgraph SubL2["📁 tokio-util/src/task (Level 2)"]
        L2_Dtokio_util_src_task_N0
        L2_Dtokio_util_src_task_N1
        L2_Dtokio_util_src_task_N2
        L2_Dtokio_util_src_task_N3
        L2_Dtokio_util_src_task_N4
        L2_Dtokio_util_src_task_N5
        L2_Dtokio_util_src_task_N6
        L2_Dtokio_util_src_task_N7
        L2_Dtokio_util_src_task_N8
        L2_Dtokio_util_src_task_N9
    end
    subgraph SubL2["📁 tokio/src/process (Level 2)"]
        L2_Dtokio_src_process_N0
        L2_Dtokio_src_process_N1
        L2_Dtokio_src_process_N2
        L2_Dtokio_src_process_N3
        L2_Dtokio_src_process_N4
        L2_Dtokio_src_process_N5
        L2_Dtokio_src_process_N6
        L2_Dtokio_src_process_N7
        L2_Dtokio_src_process_N8
        L2_Dtokio_src_process_N9
    end
    subgraph SubL2["📁 tokio/src/runtime (Level 2)"]
        L2_Dtokio_src_runtime_N0
        L2_Dtokio_src_runtime_N1
        L2_Dtokio_src_runtime_N2
        L2_Dtokio_src_runtime_N3
        L2_Dtokio_src_runtime_N4
        L2_Dtokio_src_runtime_N5
        L2_Dtokio_src_runtime_N6
        L2_Dtokio_src_runtime_N7
        L2_Dtokio_src_runtime_N8
        L2_Dtokio_src_runtime_N9
    end
    subgraph SubL2["📁 tokio/src/macros (Level 2)"]
        L2_Dtokio_src_macros_N0
        L2_Dtokio_src_macros_N1
        L2_Dtokio_src_macros_N2
        L2_Dtokio_src_macros_N3
        L2_Dtokio_src_macros_N4
    end
    subgraph SubL2["📁 tokio-util/src/io (Level 2)"]
        L2_Dtokio_util_src_io_N0
        L2_Dtokio_util_src_io_N1
        L2_Dtokio_util_src_io_N2
        L2_Dtokio_util_src_io_N3
        L2_Dtokio_util_src_io_N4
    end
    subgraph SubL2["📁 tokio-util/src/codec (Level 2)"]
        L2_Dtokio_util_src_codec_N0
        L2_Dtokio_util_src_codec_N1
        L2_Dtokio_util_src_codec_N2
        L2_Dtokio_util_src_codec_N3
        L2_Dtokio_util_src_codec_N4
        L2_Dtokio_util_src_codec_N5
        L2_Dtokio_util_src_codec_N6
        L2_Dtokio_util_src_codec_N7
        L2_Dtokio_util_src_codec_N8
        L2_Dtokio_util_src_codec_N9
    end
    subgraph SubL2["📁 tokio-util/src/sync (Level 2)"]
        L2_Dtokio_util_src_sync_N0
        L2_Dtokio_util_src_sync_N1
        L2_Dtokio_util_src_sync_N2
        L2_Dtokio_util_src_sync_N3
        L2_Dtokio_util_src_sync_N4
        L2_Dtokio_util_src_sync_N5
        L2_Dtokio_util_src_sync_N6
        L2_Dtokio_util_src_sync_N7
        L2_Dtokio_util_src_sync_N8
    end
    subgraph SubL2["📁 tokio/src/future (Level 2)"]
        L2_Dtokio_src_future_N0
        L2_Dtokio_src_future_N1
        L2_Dtokio_src_future_N2
    end
    subgraph SubL2["📁 tests-build/tests/pass (Level 2)"]
        L2_Dtests_build_tests_pass_N0
        L2_Dtests_build_tests_pass_N1
        L2_Dtests_build_tests_pass_N2
        L2_Dtests_build_tests_pass_N3
        L2_Dtests_build_tests_pass_N4
        L2_Dtests_build_tests_pass_N5
    end
    subgraph SubL2["📁 tokio/src/net (Level 2)"]
        L2_Dtokio_src_net_N0
    end
    subgraph SubL2["📁 tokio-stream/tests/support (Level 2)"]
        L2_Dtokio_stream_tests_support_N0
    end
    subgraph SubL2["📁 tokio-util/src/udp (Level 2)"]
        L2_Dtokio_util_src_udp_N0
    end
    subgraph SubL3["📁 tokio/src/runtime/scheduler/multi_thread (Level 3)"]
        L3_Dtokio_src_runtime_scheduler_multi_thread_N0
        L3_Dtokio_src_runtime_scheduler_multi_thread_N1
        L3_Dtokio_src_runtime_scheduler_multi_thread_N2
        L3_Dtokio_src_runtime_scheduler_multi_thread_N3
        L3_Dtokio_src_runtime_scheduler_multi_thread_N4
        L3_Dtokio_src_runtime_scheduler_multi_thread_N5
        L3_Dtokio_src_runtime_scheduler_multi_thread_N6
        L3_Dtokio_src_runtime_scheduler_multi_thread_N7
        L3_Dtokio_src_runtime_scheduler_multi_thread_N8
        L3_Dtokio_src_runtime_scheduler_multi_thread_N9
    end
    subgraph SubL3["📁 tokio/src/runtime/metrics/histogram (Level 3)"]
        L3_Dtokio_src_runtime_metrics_histogram_N0
        L3_Dtokio_src_runtime_metrics_histogram_N1
        L3_Dtokio_src_runtime_metrics_histogram_N2
    end
    subgraph SubL3["📁 tokio/src/runtime/scheduler/current_thread (Level 3)"]
        L3_Dtokio_src_runtime_scheduler_current_thread_N0
        L3_Dtokio_src_runtime_scheduler_current_thread_N1
        L3_Dtokio_src_runtime_scheduler_current_thread_N2
        L3_Dtokio_src_runtime_scheduler_current_thread_N3
    end
    subgraph SubL3["📁 tokio/src/runtime/time/tests (Level 3)"]
        L3_Dtokio_src_runtime_time_tests_N0
        L3_Dtokio_src_runtime_time_tests_N1
        L3_Dtokio_src_runtime_time_tests_N2
        L3_Dtokio_src_runtime_time_tests_N3
        L3_Dtokio_src_runtime_time_tests_N4
        L3_Dtokio_src_runtime_time_tests_N5
        L3_Dtokio_src_runtime_time_tests_N6
        L3_Dtokio_src_runtime_time_tests_N7
        L3_Dtokio_src_runtime_time_tests_N8
        L3_Dtokio_src_runtime_time_tests_N9
    end
    subgraph SubL3["📁 tokio/src/runtime/io/driver (Level 3)"]
        L3_Dtokio_src_runtime_io_driver_N0
    end
    subgraph SubL3["📁 tokio/src/runtime/task/trace (Level 3)"]
        L3_Dtokio_src_runtime_task_trace_N0
        L3_Dtokio_src_runtime_task_trace_N1
        L3_Dtokio_src_runtime_task_trace_N2
        L3_Dtokio_src_runtime_task_trace_N3
        L3_Dtokio_src_runtime_task_trace_N4
        L3_Dtokio_src_runtime_task_trace_N5
        L3_Dtokio_src_runtime_task_trace_N6
        L3_Dtokio_src_runtime_task_trace_N7
        L3_Dtokio_src_runtime_task_trace_N8
    end
    subgraph SubL3["📁 tokio/src/runtime/tests/loom_multi_thread (Level 3)"]
        L3_Dtokio_src_runtime_tests_loom_multi_thread_N0
        L3_Dtokio_src_runtime_tests_loom_multi_thread_N1
        L3_Dtokio_src_runtime_tests_loom_multi_thread_N2
        L3_Dtokio_src_runtime_tests_loom_multi_thread_N3
        L3_Dtokio_src_runtime_tests_loom_multi_thread_N4
        L3_Dtokio_src_runtime_tests_loom_multi_thread_N5
        L3_Dtokio_src_runtime_tests_loom_multi_thread_N6
        L3_Dtokio_src_runtime_tests_loom_multi_thread_N7
    end
    subgraph SubL3["📁 tokio/src/runtime/time/wheel (Level 3)"]
        L3_Dtokio_src_runtime_time_wheel_N0
        L3_Dtokio_src_runtime_time_wheel_N1
    end
    subgraph SubL3["📁 tokio/src/runtime/tests/loom_current_thread (Level 3)"]
        L3_Dtokio_src_runtime_tests_loom_current_thread_N0
        L3_Dtokio_src_runtime_tests_loom_current_thread_N1
    end
    subgraph SubL3["📁 tokio/src/runtime/scheduler/inject (Level 3)"]
        L3_Dtokio_src_runtime_scheduler_inject_N0
    end

    %% Styling
    classDef level1 fill:#e8f5e8,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef level2 fill:#e1f5fe,stroke:#01579b,stroke-width:2px,color:#0d47a1
    classDef level3 fill:#fff3e0,stroke:#ef6c00,stroke-width:2px,color:#e65100
    class L1_Dbenches_N0 level1
    class L1_Dbenches_N1 level1
    class L1_Dbenches_N2 level1
    class L1_Dbenches_N3 level1
    class L1_Dbenches_N4 level1
    class L1_Dbenches_N5 level1
    class L1_Dbenches_N6 level1
    class L1_Dbenches_N7 level1
    class L1_Dbenches_N8 level1
    class L1_Dbenches_N9 level1
    class L1_Dexamples_N0 level1
    class L1_Dexamples_N1 level1
    class L1_Dexamples_N2 level1
    class L1_Dexamples_N3 level1
    class L1_Dexamples_N4 level1
    class L1_Dexamples_N5 level1
    class L1_Dexamples_N6 level1
    class L1_Dexamples_N7 level1
    class L1_Dexamples_N8 level1
    class L1_Dexamples_N9 level1
    class L1_Dsrc_N0 level1
    class L1_Dsrc_N1 level1
    class L1_Dsrc_N2 level1
    class L1_Dsrc_N3 level1
    class L1_Dsrc_N4 level1
    class L1_Dsrc_N5 level1
    class L1_Dsrc_N6 level1
    class L1_D._N0 level1
    class L1_D._N1 level1
    class L2_Dtests_build_tests_fail_N0 level2
    class L2_Dtests_build_tests_fail_N1 level2
    class L2_Dtests_build_tests_fail_N2 level2
    class L2_Dtests_build_tests_fail_N3 level2
    class L2_Dtests_build_tests_fail_N4 level2
    class L2_Dtests_build_tests_fail_N5 level2
    class L2_Dtests_build_tests_fail_N6 level2
    class L2_Dtests_build_tests_fail_N7 level2
    class L2_Dtests_build_tests_fail_N8 level2
    class L2_Dtests_build_tests_fail_N9 level2
    class L2_Dtokio_stream_src_wrappers_N0 level2
    class L2_Dtokio_stream_src_wrappers_N1 level2
    class L2_Dtokio_stream_src_wrappers_N2 level2
    class L2_Dtokio_stream_src_wrappers_N3 level2
    class L2_Dtokio_stream_src_wrappers_N4 level2
    class L2_Dtokio_stream_src_wrappers_N5 level2
    class L2_Dtokio_stream_src_wrappers_N6 level2
    class L2_Dtokio_stream_src_wrappers_N7 level2
    class L2_Dtokio_stream_src_wrappers_N8 level2
    class L2_Dtokio_stream_src_wrappers_N9 level2
    class L2_Dtokio_src_task_N0 level2
    class L2_Dtokio_src_task_N1 level2
    class L2_Dtokio_src_task_N2 level2
    class L2_Dtokio_src_task_N3 level2
    class L2_Dtokio_src_task_N4 level2
    class L2_Dtokio_src_task_N5 level2
    class L2_Dtokio_src_task_N6 level2
    class L2_Dtokio_src_task_N7 level2
    class L2_Dtokio_src_task_N8 level2
    class L2_Dtokio_src_task_N9 level2
    class L2_Dtokio_src_util_N0 level2
    class L2_Dtokio_src_util_N1 level2
    class L2_Dtokio_src_util_N2 level2
    class L2_Dtokio_src_util_N3 level2
    class L2_Dtokio_src_util_N4 level2
    class L2_Dtokio_src_util_N5 level2
    class L2_Dtokio_src_util_N6 level2
    class L2_Dtokio_src_util_N7 level2
    class L2_Dtokio_src_util_N8 level2
    class L2_Dtokio_src_util_N9 level2
    class L2_Dtokio_src_io_N0 level2
    class L2_Dtokio_src_io_N1 level2
    class L2_Dtokio_src_io_N2 level2
    class L2_Dtokio_src_io_N3 level2
    class L2_Dtokio_src_io_N4 level2
    class L2_Dtokio_src_io_N5 level2
    class L2_Dtokio_src_io_N6 level2
    class L2_Dtokio_src_io_N7 level2
    class L2_Dtokio_src_io_N8 level2
    class L2_Dtokio_src_io_N9 level2
    class L2_Dtokio_src_sync_N0 level2
    class L2_Dtokio_src_sync_N1 level2
    class L2_Dtokio_src_sync_N2 level2
    class L2_Dtokio_src_sync_N3 level2
    class L2_Dtokio_src_sync_N4 level2
    class L2_Dtokio_src_sync_N5 level2
    class L2_Dtokio_src_sync_N6 level2
    class L2_Dtokio_src_sync_N7 level2
    class L2_Dtokio_src_sync_N8 level2
    class L2_Dtokio_src_sync_N9 level2
    class L2_Dtokio_src_fs_N0 level2
    class L2_Dtokio_src_fs_N1 level2
    class L2_Dtokio_src_fs_N2 level2
    class L2_Dtokio_src_fs_N3 level2
    class L2_Dtokio_src_fs_N4 level2
    class L2_Dtokio_src_fs_N5 level2
    class L2_Dtokio_src_fs_N6 level2
    class L2_Dtokio_src_fs_N7 level2
    class L2_Dtokio_src_fs_N8 level2
    class L2_Dtokio_src_fs_N9 level2
    class L2_Dtokio_stream_src_stream_ext_N0 level2
    class L2_Dtokio_stream_src_stream_ext_N1 level2
    class L2_Dtokio_stream_src_stream_ext_N2 level2
    class L2_Dtokio_stream_src_stream_ext_N3 level2
    class L2_Dtokio_stream_src_stream_ext_N4 level2
    class L2_Dtokio_src_signal_N0 level2
    class L2_Dtokio_src_signal_N1 level2
    class L2_Dtokio_src_signal_N2 level2
    class L2_Dtokio_src_signal_N3 level2
    class L2_Dtokio_src_signal_N4 level2
    class L2_Dtokio_src_signal_N5 level2
    class L2_Dtokio_src_signal_N6 level2
    class L2_Dtokio_src_signal_N7 level2
    class L2_Dtokio_src_signal_N8 level2
    class L2_Dtokio_src_signal_N9 level2
    class L2_Dtokio_util_src_util_N0 level2
    class L2_Dtokio_util_src_util_N1 level2
    class L2_Dtokio_util_src_util_N2 level2
    class L2_Dtokio_util_src_util_N3 level2
    class L2_Dtokio_util_src_time_N0 level2
    class L2_Dtokio_util_src_time_N1 level2
    class L2_Dtokio_util_src_time_N2 level2
    class L2_Dtokio_util_src_time_N3 level2
    class L2_Dtokio_util_src_time_N4 level2
    class L2_Dtokio_util_src_time_N5 level2
    class L2_Dtokio_util_src_time_N6 level2
    class L2_Dtokio_util_src_time_N7 level2
    class L2_Dtokio_util_src_net_N0 level2
    class L2_Dtokio_util_src_net_N1 level2
    class L2_Dtokio_tests_support_N0 level2
    class L2_Dtokio_tests_support_N1 level2
    class L2_Dtokio_tests_support_N2 level2
    class L2_Dtokio_tests_support_N3 level2
    class L2_Dtokio_tests_support_N4 level2
    class L2_Dtokio_tests_support_N5 level2
    class L2_Dtokio_tests_support_N6 level2
    class L2_Dtokio_tests_support_N7 level2
    class L2_Dtokio_src_time_N0 level2
    class L2_Dtokio_src_time_N1 level2
    class L2_Dtokio_src_time_N2 level2
    class L2_Dtokio_src_time_N3 level2
    class L2_Dtokio_src_time_N4 level2
    class L2_Dtokio_src_time_N5 level2
    class L2_Dtokio_src_time_N6 level2
    class L2_Dtokio_src_time_N7 level2
    class L2_Dtokio_src_time_N8 level2
    class L2_Dtokio_src_time_N9 level2
    class L2_Dtokio_util_src_task_N0 level2
    class L2_Dtokio_util_src_task_N1 level2
    class L2_Dtokio_util_src_task_N2 level2
    class L2_Dtokio_util_src_task_N3 level2
    class L2_Dtokio_util_src_task_N4 level2
    class L2_Dtokio_util_src_task_N5 level2
    class L2_Dtokio_util_src_task_N6 level2
    class L2_Dtokio_util_src_task_N7 level2
    class L2_Dtokio_util_src_task_N8 level2
    class L2_Dtokio_util_src_task_N9 level2
    class L2_Dtokio_src_process_N0 level2
    class L2_Dtokio_src_process_N1 level2
    class L2_Dtokio_src_process_N2 level2
    class L2_Dtokio_src_process_N3 level2
    class L2_Dtokio_src_process_N4 level2
    class L2_Dtokio_src_process_N5 level2
    class L2_Dtokio_src_process_N6 level2
    class L2_Dtokio_src_process_N7 level2
    class L2_Dtokio_src_process_N8 level2
    class L2_Dtokio_src_process_N9 level2
    class L2_Dtokio_src_runtime_N0 level2
    class L2_Dtokio_src_runtime_N1 level2
    class L2_Dtokio_src_runtime_N2 level2
    class L2_Dtokio_src_runtime_N3 level2
    class L2_Dtokio_src_runtime_N4 level2
    class L2_Dtokio_src_runtime_N5 level2
    class L2_Dtokio_src_runtime_N6 level2
    class L2_Dtokio_src_runtime_N7 level2
    class L2_Dtokio_src_runtime_N8 level2
    class L2_Dtokio_src_runtime_N9 level2
    class L2_Dtokio_src_macros_N0 level2
    class L2_Dtokio_src_macros_N1 level2
    class L2_Dtokio_src_macros_N2 level2
    class L2_Dtokio_src_macros_N3 level2
    class L2_Dtokio_src_macros_N4 level2
    class L2_Dtokio_util_src_io_N0 level2
    class L2_Dtokio_util_src_io_N1 level2
    class L2_Dtokio_util_src_io_N2 level2
    class L2_Dtokio_util_src_io_N3 level2
    class L2_Dtokio_util_src_io_N4 level2
    class L2_Dtokio_util_src_codec_N0 level2
    class L2_Dtokio_util_src_codec_N1 level2
    class L2_Dtokio_util_src_codec_N2 level2
    class L2_Dtokio_util_src_codec_N3 level2
    class L2_Dtokio_util_src_codec_N4 level2
    class L2_Dtokio_util_src_codec_N5 level2
    class L2_Dtokio_util_src_codec_N6 level2
    class L2_Dtokio_util_src_codec_N7 level2
    class L2_Dtokio_util_src_codec_N8 level2
    class L2_Dtokio_util_src_codec_N9 level2
    class L2_Dtokio_util_src_sync_N0 level2
    class L2_Dtokio_util_src_sync_N1 level2
    class L2_Dtokio_util_src_sync_N2 level2
    class L2_Dtokio_util_src_sync_N3 level2
    class L2_Dtokio_util_src_sync_N4 level2
    class L2_Dtokio_util_src_sync_N5 level2
    class L2_Dtokio_util_src_sync_N6 level2
    class L2_Dtokio_util_src_sync_N7 level2
    class L2_Dtokio_util_src_sync_N8 level2
    class L2_Dtokio_src_future_N0 level2
    class L2_Dtokio_src_future_N1 level2
    class L2_Dtokio_src_future_N2 level2
    class L2_Dtests_build_tests_pass_N0 level2
    class L2_Dtests_build_tests_pass_N1 level2
    class L2_Dtests_build_tests_pass_N2 level2
    class L2_Dtests_build_tests_pass_N3 level2
    class L2_Dtests_build_tests_pass_N4 level2
    class L2_Dtests_build_tests_pass_N5 level2
    class L2_Dtokio_src_net_N0 level2
    class L2_Dtokio_stream_tests_support_N0 level2
    class L2_Dtokio_util_src_udp_N0 level2
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N0 level3
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N1 level3
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N2 level3
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N3 level3
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N4 level3
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N5 level3
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N6 level3
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N7 level3
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N8 level3
    class L3_Dtokio_src_runtime_scheduler_multi_thread_N9 level3
    class L3_Dtokio_src_runtime_metrics_histogram_N0 level3
    class L3_Dtokio_src_runtime_metrics_histogram_N1 level3
    class L3_Dtokio_src_runtime_metrics_histogram_N2 level3
    class L3_Dtokio_src_runtime_scheduler_current_thread_N0 level3
    class L3_Dtokio_src_runtime_scheduler_current_thread_N1 level3
    class L3_Dtokio_src_runtime_scheduler_current_thread_N2 level3
    class L3_Dtokio_src_runtime_scheduler_current_thread_N3 level3
    class L3_Dtokio_src_runtime_time_tests_N0 level3
    class L3_Dtokio_src_runtime_time_tests_N1 level3
    class L3_Dtokio_src_runtime_time_tests_N2 level3
    class L3_Dtokio_src_runtime_time_tests_N3 level3
    class L3_Dtokio_src_runtime_time_tests_N4 level3
    class L3_Dtokio_src_runtime_time_tests_N5 level3
    class L3_Dtokio_src_runtime_time_tests_N6 level3
    class L3_Dtokio_src_runtime_time_tests_N7 level3
    class L3_Dtokio_src_runtime_time_tests_N8 level3
    class L3_Dtokio_src_runtime_time_tests_N9 level3
    class L3_Dtokio_src_runtime_io_driver_N0 level3
    class L3_Dtokio_src_runtime_task_trace_N0 level3
    class L3_Dtokio_src_runtime_task_trace_N1 level3
    class L3_Dtokio_src_runtime_task_trace_N2 level3
    class L3_Dtokio_src_runtime_task_trace_N3 level3
    class L3_Dtokio_src_runtime_task_trace_N4 level3
    class L3_Dtokio_src_runtime_task_trace_N5 level3
    class L3_Dtokio_src_runtime_task_trace_N6 level3
    class L3_Dtokio_src_runtime_task_trace_N7 level3
    class L3_Dtokio_src_runtime_task_trace_N8 level3
    class L3_Dtokio_src_runtime_tests_loom_multi_thread_N0 level3
    class L3_Dtokio_src_runtime_tests_loom_multi_thread_N1 level3
    class L3_Dtokio_src_runtime_tests_loom_multi_thread_N2 level3
    class L3_Dtokio_src_runtime_tests_loom_multi_thread_N3 level3
    class L3_Dtokio_src_runtime_tests_loom_multi_thread_N4 level3
    class L3_Dtokio_src_runtime_tests_loom_multi_thread_N5 level3
    class L3_Dtokio_src_runtime_tests_loom_multi_thread_N6 level3
    class L3_Dtokio_src_runtime_tests_loom_multi_thread_N7 level3
    class L3_Dtokio_src_runtime_time_wheel_N0 level3
    class L3_Dtokio_src_runtime_time_wheel_N1 level3
    class L3_Dtokio_src_runtime_tests_loom_current_thread_N0 level3
    class L3_Dtokio_src_runtime_tests_loom_current_thread_N1 level3
    class L3_Dtokio_src_runtime_scheduler_inject_N0 level3
```

---

*⬅️ Back to: [Overview](index.md) | 🗂️ Full Data: [JSON Export](data/full_isg.json)*
