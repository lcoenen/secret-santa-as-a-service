const fetch = require('node-fetch');
const toml = require('toml');

const users = [
	`
username = "tom" 
password = "abcdef"
email = "tom@email.com"
`,
	`
username = "jerry"
password = "ghijklm"
email = "jerry@email.com"
`,
	`
username = "jean"
password = "nopqrst"
email = "jean@email.com"
`,
];

const HOST = 'http://localhost:3000';

describe('happy paths', () => {
	it('should subscribe, start and check', () =>
		Promise.all(
			users.map(user =>
				fetch(`${HOST}/subscribe`, {
					method: 'POST',
					body: user,
				})
					.then(res => res.text())
					.then(response => expect(response).toBe('Subscribed')),
			),
		)
			.then(() =>
				fetch(`${HOST}/start`)
					.then(res => res.text())
					.then(response => expect(response).toBe('Started')),
			)
			.then(() =>
				Promise.all(
					users.map(user =>
						fetch(`${HOST}/check`)
							.then(res => res.text())
							.then(response => toml.parse(response))
							.then(({ partner }) => {
								expect(
									users.map(toml.parse).map(user => user.usename),
								).toContain(partner);
								expect(partner).not.toBe(toml.parse(user).username);
							}),
					),
				),
			));
});
