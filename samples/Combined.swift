protocol Combined {
	// POST /{path}?q=:query
	func post(for query: String, with path: String, body: Hello) async throws -> Hello
}
