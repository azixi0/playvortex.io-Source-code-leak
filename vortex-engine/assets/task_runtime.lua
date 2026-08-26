-- leaked by @azixi0 on github
local __schedule = __schedule
local __report = __report

local MAX_HANDLERS = 512

local function suspended(co, first)
	if coroutine.status(co) ~= "suspended" then
		return false
	end
	if type(first) == "table" and first.__block then
		return true
	end
	local seconds = 0
	if type(first) == "table" and first.__wait then
		seconds = first.__wait
	end
	__schedule(co, seconds)
	return true
end

local function adopt(co, ok, first)
	if not ok then
		error(first, 0)
	end
	suspended(co, first)
	return co
end

local function adoptSafe(co, ok, first)
	if not ok then
		__report(tostring(first))
		return co
	end
	suspended(co, first)
	return co
end

local function bind(fn, ...)
	local args = table.pack(...)
	if args.n == 0 then
		return fn
	end
	return function()
		return fn(table.unpack(args, 1, args.n))
	end
end

task = {}

function task.wait(seconds)
	return coroutine.yield({ __wait = seconds or 0 })
end

function task.spawn(fn, ...)
	local co = fn
	if type(fn) == "function" then
		co = coroutine.create(fn)
	end
	return adopt(co, coroutine.resume(co, ...))
end

function task.delay(seconds, fn, ...)
	local co = coroutine.create(bind(fn, ...))
	__schedule(co, seconds or 0)
	return co
end

function task.defer(fn, ...)
	return task.delay(0, fn, ...)
end

function wait(seconds)
	return task.wait(seconds)
end

function spawn(fn)
	return task.spawn(fn)
end

function delay(seconds, fn)
	return task.delay(seconds, fn)
end

local Connection = {}
Connection.__index = Connection

function Connection:Disconnect()
	if not self.Connected then
		return
	end
	self.Connected = false
	local handlers = self._signal._handlers
	for i, c in ipairs(handlers) do
		if c == self then
			table.remove(handlers, i)
			break
		end
	end
end

Connection.disconnect = Connection.Disconnect

local Signal = {}
Signal.__index = Signal

function Signal.new(name)
	return setmetatable({ _name = name, _handlers = {}, _waiting = {} }, Signal)
end

function Signal:Connect(fn)
	if type(fn) ~= "function" then
		error("Connect expects a function", 2)
	end
	if #self._handlers >= MAX_HANDLERS then
		error(`signal "{self._name}" exceeded {MAX_HANDLERS} connections`, 2)
	end
	local conn = setmetatable({ Connected = true, _signal = self, _fn = fn }, Connection)
	table.insert(self._handlers, conn)
	return conn
end

Signal.connect = Signal.Connect

function Signal:Once(fn)
	local conn
	conn = self:Connect(function(...)
		conn:Disconnect()
		fn(...)
	end)
	return conn
end

function Signal:Wait()
	table.insert(self._waiting, coroutine.running())
	return coroutine.yield({ __block = true })
end

function Signal:Fire(...)
	local snapshot = {}
	for i, conn in ipairs(self._handlers) do
		snapshot[i] = conn
	end
	for _, conn in ipairs(snapshot) do
		if conn.Connected then
			local co = coroutine.create(conn._fn)
			adoptSafe(co, coroutine.resume(co, ...))
		end
	end

	local waiters = self._waiting
	if #waiters > 0 then
		self._waiting = {}
		for _, co in ipairs(waiters) do
			adoptSafe(co, coroutine.resume(co, ...))
		end
	end
end

Signal.fire = Signal.Fire

local registry = {}

function __signal(key)
	local existing = registry[key]
	if existing then
		return existing
	end
	local created = Signal.new(key)
	registry[key] = created
	return created
end

function __fire(key, ...)
	local existing = registry[key]
	if existing then
		existing:Fire(...)
	end
end

function __has(key)
	local existing = registry[key]
	return existing ~= nil and (#existing._handlers > 0 or #existing._waiting > 0)
end

RunService = {
	Heartbeat = __signal("RunService.Heartbeat"),
}
