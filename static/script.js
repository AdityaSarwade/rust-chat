let roomListDiv = document.getElementById("room-list");
let messagesDiv = document.getElementById("messages");
let newMessageForm = document.getElementById("new-message");
let newRoomForm = document.getElementById("new-room");
let statusDiv = document.getElementById("status");

let roomTemplate = document.getElementById("room");
let messageTemplate = document.getElementById("message");

let messageField = newMessageForm.querySelector("#message");
let usernameField = newMessageForm.querySelector("#username");
let roomNameField = newRoomForm.querySelector("#name");

let loginForm = document.getElementById("login-form");
let registerForm = document.getElementById("register-form");
let logoutButton = document.getElementById("logout-button");

var STATE = {
	room: "ChatRoom 1",
	rooms: {},
	connected: false,
	token: null,
};

// Handle registration
registerForm.addEventListener("submit", async (e) => {
	e.preventDefault();
	const username = document.getElementById("register-username").value;
	const password = document.getElementById("register-password").value;
	const mail = document.getElementById("register-mail").value;

	const response = await fetch("/registration", {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify({ username, password, mail }),
	});

	if (response.ok) {
		alert("Registration successful! Please log in.");
	} else {
		alert("Registration failed.");
	}
});

// Handle login
loginForm.addEventListener("submit", async (e) => {
	e.preventDefault();
	const username = document.getElementById("login-username").value;
	const password = document.getElementById("login-password").value;

	const response = await fetch("/login", {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify({ username, password }),
	});

	if (response.ok) {
		const data = await response.json();
		STATE.token = data.token;
		localStorage.setItem("token", STATE.token);
		toggleAuthForms(true);
	} else {
		alert("Login failed.");
	}
});

// Handle logout
logoutButton.addEventListener("click", () => {
	STATE.token = null;
	localStorage.removeItem("token");
	toggleAuthForms(false);
});

// Toggle auth forms visibility
function toggleAuthForms(isLoggedIn) {
	if (isLoggedIn) {
		loginForm.style.display = "none";
		registerForm.style.display = "none";
		logoutButton.style.display = "block";
	} else {
		loginForm.style.display = "block";
		registerForm.style.display = "block";
		logoutButton.style.display = "none";
	}
}

// Initialize auth state from local storage
function initAuth() {
	const token = localStorage.getItem("token");
	if (token) {
		STATE.token = token;
		toggleAuthForms(true);
		subscribe("/events");
	} else {
		toggleAuthForms(false);
	}
}

// Modify fetch function to include token
function authFetch(url, options = {}) {
	options.headers = {
		...options.headers,
		Authorization: `Bearer ${STATE.token}`,
	};
	return fetch(url, options);
}

// Generate a color from a "hash" of a string. Thanks, internet.
function hashColor(str) {
	let hash = 0;
	for (var i = 0; i < str.length; i++) {
		hash = str.charCodeAt(i) + ((hash << 5) - hash);
		hash = hash & hash;
	}

	return `hsl(${hash % 360}, 100%, 70%)`;
}

// Add a new room `name` and change to it. Returns `true` if the room didn't
// already exist and false otherwise.
function addRoom(name) {
	if (STATE[name]) {
		changeRoom(name);
		return false;
	}

	var node = roomTemplate.content.cloneNode(true);
	var room = node.querySelector(".room");
	room.addEventListener("click", () => changeRoom(name));
	room.textContent = name;
	room.dataset.name = name;
	roomListDiv.appendChild(node);

	STATE[name] = [];
	changeRoom(name);
	return true;
}

// Change the current room to `name`, restoring its messages.
function changeRoom(name) {
	if (STATE.room == name) return;

	var newRoom = roomListDiv.querySelector(`.room[data-name='${name}']`);
	var oldRoom = roomListDiv.querySelector(`.room[data-name='${STATE.room}']`);
	if (!newRoom || !oldRoom) return;

	STATE.room = name;
	oldRoom.classList.remove("active");
	newRoom.classList.add("active");

	messagesDiv.querySelectorAll(".message").forEach((msg) => {
		messagesDiv.removeChild(msg);
	});

	STATE[name].forEach((data) =>
		addMessage(name, data.username, data.message)
	);
}

// Add `message` from `username` to `room`. If `push`, then actually store the
// message. If the current room is `room`, render the message.
function addMessage(room, username, message, push = false) {
	if (push) {
		STATE[room].push({ username, message });
	}

	if (STATE.room == room) {
		var node = messageTemplate.content.cloneNode(true);
		node.querySelector(".message .username").textContent = username;
		node.querySelector(".message .username").style.color =
			hashColor(username);
		node.querySelector(".message .text").innerHTML = message;
		messagesDiv.appendChild(node);
	}
}

// Subscribe to the event source at `uri` with exponential backoff reconnect.
function subscribe(uri) {
	var retryTime = 1;

	function connect(uri) {
		const events = new EventSource(uri);

		events.addEventListener("message", (ev) => {
			console.log("raw data", JSON.stringify(ev.data));
			console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
			const msg = JSON.parse(ev.data);
			if (!("message" in msg) || !("room" in msg) || !("username" in msg))
				return;
			addMessage(msg.room, msg.username, msg.message, true);
		});

		events.addEventListener("open", () => {
			setConnectedStatus(true);
			console.log(`connected to event stream at ${uri}`);
			retryTime = 1;
		});

		events.addEventListener("error", () => {
			setConnectedStatus(false);
			events.close();

			let timeout = retryTime;
			retryTime = Math.min(64, retryTime * 2);
			console.log(
				`connection lost. attempting to reconnect in ${timeout}s`
			);
			setTimeout(() => connect(uri), (() => timeout * 1000)());
		});
	}

	connect(uri);
}

// Set the connection status: `true` for connected, `false` for disconnected.
function setConnectedStatus(status) {
	STATE.connected = status;
	statusDiv.className = status ? "connected" : "reconnecting";
}

// Let's go! Initialize the world.
function init() {
	// Initialize some rooms.
	addRoom("ChatRoom 1");
	addRoom("ChatRoom 2");
	changeRoom("ChatRoom 1");
	addMessage(
		"ChatRoom 1",
		"Assistant",
		"Hey! Open another browser tab, send a message.",
		true
	);
	addMessage(
		"ChatRoom 2",
		"Assistant",
		"This is another room. Create your own too!",
		true
	);

	// Set up the form handler.
	newMessageForm.addEventListener("submit", (e) => {
		e.preventDefault();

		const room = STATE.room;
		const message = messageField.value;
		const username = usernameField.value || "guest";
		if (!message || !username) return;

		if (STATE.connected) {
			fetch("/message", {
				method: "POST",
				body: new URLSearchParams({ room, username, message }),
			}).then((response) => {
				if (response.ok) messageField.value = "";
			});
		}
	});

	// Set up the new room handler.
	newRoomForm.addEventListener("submit", (e) => {
		e.preventDefault();

		const room = roomNameField.value;
		if (!room) return;

		roomNameField.value = "";
		if (!addRoom(room)) return;

		addMessage(
			room,
			"Assistant",
			`Look, your own "${room}" room! Nice.`,
			true
		);
	});

	// Subscribe to server-sent events.
	subscribe("/events");
}

init();
