protocol Headers {
	// GET /get
    // Content-Type: application/json
	// Custom: {value}
	func get(for value: String) async throws
}
