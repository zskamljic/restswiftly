protocol QueryParameter {
	// GET /get?q=:query
	func get(query: String) async throws

	// GET /get?q=:query
	func get(for query: String) async throws
}
