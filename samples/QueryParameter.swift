protocol QueryParameter {
	// GET /get?q=:query
	func get(query: String) async throws
}
