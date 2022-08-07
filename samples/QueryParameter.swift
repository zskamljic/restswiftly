protocol QueryParameter {
	// GET /get?q=:query
	func get(query: String) async throws

	// GET /get?q=:query&q2=something
	func get(for query: String) async throws
}
