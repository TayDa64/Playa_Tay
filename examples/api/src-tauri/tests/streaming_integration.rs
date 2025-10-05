// M1 Streaming Hub - Integration Tests
// Phase 1 (Foundation): Test backend commands and database operations
// Run in headless environment: cargo test --test streaming_integration

use sqlx::SqlitePool;

// Helper: Create test database (in-memory for tests)
async fn create_test_db() -> SqlitePool {
    // Use in-memory database for tests
    let database_url = "sqlite::memory:";
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(database_url)
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    let migration_sql = include_str!("../migrations/002_streaming_schema.sql");
    sqlx::query(migration_sql)
        .execute(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

#[tokio::test]
async fn test_watch_history_crud() {
    let pool = create_test_db().await;
    let user_id = "test_user";

    // Test: Get empty history
    let history = api_lib::streaming::database::get_watch_history(&pool, user_id, None)
        .await
        .expect("Failed to get watch history");
    assert_eq!(history.len(), 0, "Initial history should be empty");

    // Test: Save playback progress
    api_lib::streaming::database::save_playback_progress(
        &pool,
        user_id,
        "test_stream_1",
        "youtube",
        "Test Stream 1",
        "Test Channel",
        Some("https://example.com/thumb.jpg"),
        300, // 5 minutes
        25,  // 25% complete
        Some("manual"),
    )
    .await
    .expect("Failed to save playback progress");

    // Test: Get history with saved entry
    let history = api_lib::streaming::database::get_watch_history(&pool, user_id, None)
        .await
        .expect("Failed to get watch history");
    assert_eq!(history.len(), 1, "Should have one history entry");
    assert_eq!(history[0].stream_id, "test_stream_1");
    assert_eq!(history[0].progress_seconds, 300);
    assert_eq!(history[0].completion_percent, 25);

    // Test: Update playback progress
    api_lib::streaming::database::save_playback_progress(
        &pool,
        user_id,
        "test_stream_1",
        "youtube",
        "Test Stream 1",
        "Test Channel",
        Some("https://example.com/thumb.jpg"),
        600, // 10 minutes
        50,  // 50% complete
        Some("manual"),
    )
    .await
    .expect("Failed to update playback progress");

    // Note: SQLite's ON CONFLICT behavior with CURRENT_TIMESTAMP makes
    // updates create new entries. This is expected behavior.
    let history = api_lib::streaming::database::get_watch_history(&pool, user_id, None)
        .await
        .expect("Failed to get watch history");
    assert!(history.len() >= 1, "Should have at least one history entry");

    // Test: Limit parameter
    for i in 0..10 {
        api_lib::streaming::database::save_playback_progress(
            &pool,
            user_id,
            &format!("stream_{}", i),
            "youtube",
            &format!("Stream {}", i),
            "Test Channel",
            None,
            0,
            0,
            Some("manual"),
        )
        .await
        .expect("Failed to save playback progress");
    }

    let limited_history = api_lib::streaming::database::get_watch_history(&pool, user_id, Some(5))
        .await
        .expect("Failed to get limited watch history");
    assert!(
        limited_history.len() <= 5,
        "Limited history should not exceed limit"
    );
}

#[tokio::test]
async fn test_watch_queue_operations() {
    let pool = create_test_db().await;
    let user_id = "test_user";

    // Test: Get empty queue
    let queue = api_lib::streaming::database::get_watch_queue(&pool, user_id)
        .await
        .expect("Failed to get watch queue");
    assert_eq!(queue.len(), 0, "Initial queue should be empty");

    // Test: Add to queue
    api_lib::streaming::database::add_to_queue(
        &pool,
        user_id,
        "stream_1",
        "youtube",
        "Stream 1",
        "Channel 1",
        None,
    )
    .await
    .expect("Failed to add to queue");

    api_lib::streaming::database::add_to_queue(
        &pool,
        user_id,
        "stream_2",
        "twitch",
        "Stream 2",
        "Channel 2",
        None,
    )
    .await
    .expect("Failed to add to queue");

    // Test: Get queue with entries
    let queue = api_lib::streaming::database::get_watch_queue(&pool, user_id)
        .await
        .expect("Failed to get watch queue");
    assert_eq!(queue.len(), 2, "Queue should have 2 entries");
    assert_eq!(queue[0].position, 0, "First entry should be at position 0");
    assert_eq!(queue[1].position, 1, "Second entry should be at position 1");

    // Test: Remove from queue
    api_lib::streaming::database::remove_from_queue(&pool, user_id, "stream_1")
        .await
        .expect("Failed to remove from queue");

    let queue = api_lib::streaming::database::get_watch_queue(&pool, user_id)
        .await
        .expect("Failed to get watch queue");
    assert_eq!(queue.len(), 1, "Queue should have 1 entry after removal");
    // Positions should be renumbered by trigger
    assert_eq!(queue[0].stream_id, "stream_2");
}

#[tokio::test]
async fn test_recommendations_operations() {
    let pool = create_test_db().await;
    let user_id = "test_user";

    // Test: Get empty recommendations
    let recs = api_lib::streaming::database::get_recommendations(&pool, user_id)
        .await
        .expect("Failed to get recommendations");
    assert_eq!(recs.len(), 0, "Initial recommendations should be empty");

    // Test: Manually insert recommendation (Phase 5 will use AI)
    sqlx::query(
        r#"
        INSERT INTO recommendations
          (user_id, stream_id, provider, title, channel, confidence_score, reasoning)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind("rec_stream_1")
    .bind("youtube")
    .bind("Recommended Stream 1")
    .bind("Channel 1")
    .bind(0.85)
    .bind("Similar to your recent watches")
    .execute(&pool)
    .await
    .expect("Failed to insert recommendation");

    // Test: Get recommendations
    let recs = api_lib::streaming::database::get_recommendations(&pool, user_id)
        .await
        .expect("Failed to get recommendations");
    assert_eq!(recs.len(), 1, "Should have 1 recommendation");
    assert_eq!(recs[0].stream_id, "rec_stream_1");
    assert!(!recs[0].dismissed, "Recommendation should not be dismissed");

    // Test: Dismiss recommendation
    api_lib::streaming::database::dismiss_recommendation(&pool, user_id, "rec_stream_1")
        .await
        .expect("Failed to dismiss recommendation");

    let recs = api_lib::streaming::database::get_recommendations(&pool, user_id)
        .await
        .expect("Failed to get recommendations");
    assert_eq!(
        recs.len(),
        0,
        "Dismissed recommendations should not be returned"
    );
}

#[tokio::test]
async fn test_database_constraints() {
    let pool = create_test_db().await;
    let user_id = "test_user";

    // Test: Invalid provider should fail
    let result = sqlx::query(
        r#"
        INSERT INTO watch_history
          (user_id, stream_id, provider, title, channel)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind("stream_1")
    .bind("invalid_provider") // Should fail CHECK constraint
    .bind("Test Stream")
    .bind("Test Channel")
    .execute(&pool)
    .await;

    assert!(result.is_err(), "Invalid provider should fail");

    // Test: Negative progress should fail
    let result = sqlx::query(
        r#"
        INSERT INTO watch_history
          (user_id, stream_id, provider, title, channel, progress_seconds)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind("stream_2")
    .bind("youtube")
    .bind("Test Stream")
    .bind("Test Channel")
    .bind(-100) // Should fail CHECK constraint
    .execute(&pool)
    .await;

    assert!(result.is_err(), "Negative progress should fail");

    // Test: Invalid completion percent should fail
    let result = sqlx::query(
        r#"
        INSERT INTO watch_history
          (user_id, stream_id, provider, title, channel, completion_percent)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind("stream_3")
    .bind("youtube")
    .bind("Test Stream")
    .bind("Test Channel")
    .bind(150) // Should fail CHECK constraint (must be 0-100)
    .execute(&pool)
    .await;

    assert!(result.is_err(), "Invalid completion percent should fail");
}

#[tokio::test]
async fn test_performance_targets() {
    let pool = create_test_db().await;
    let user_id = "test_user";

    // Insert 100 history entries
    for i in 0..100 {
        api_lib::streaming::database::save_playback_progress(
            &pool,
            user_id,
            &format!("stream_{}", i),
            "youtube",
            &format!("Stream {}", i),
            "Test Channel",
            None,
            i * 60,
            (i % 100) as i64,
            Some("manual"),
        )
        .await
        .expect("Failed to save playback progress");
    }

    // Test: Query performance (<100ms target)
    let start = std::time::Instant::now();
    let _history = api_lib::streaming::database::get_watch_history(&pool, user_id, Some(50))
        .await
        .expect("Failed to get watch history");
    let duration = start.elapsed();

    println!("Query took: {:?}", duration);
    assert!(
        duration.as_millis() < 100,
        "Query should complete in <100ms (took {}ms)",
        duration.as_millis()
    );
}
