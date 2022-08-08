protocol Failing {
	// GET /get
	func get(query: String) async throws
}