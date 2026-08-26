-- leaked by @azixi0 on github
Vector3 = {}
Vector3.__index = Vector3

function Vector3.new(x, y, z)
    return setmetatable({ x = x or 0, y = y or 0, z = z or 0 }, Vector3)
end

function Vector3.__add(a, b)
    return Vector3.new(a.x + b.x, a.y + b.y, a.z + b.z)
end

function Vector3.__sub(a, b)
    return Vector3.new(a.x - b.x, a.y - b.y, a.z - b.z)
end

function Vector3.__mul(a, b)
    if type(a) == "number" then
        return Vector3.new(a * b.x, a * b.y, a * b.z)
    end
    if type(b) == "number" then
        return Vector3.new(a.x * b, a.y * b, a.z * b)
    end
    return Vector3.new(a.x * b.x, a.y * b.y, a.z * b.z)
end

function Vector3.__div(a, b)
    if type(b) == "number" then
        return Vector3.new(a.x / b, a.y / b, a.z / b)
    end
    return Vector3.new(a.x / b.x, a.y / b.y, a.z / b.z)
end

function Vector3.__unm(v)
    return Vector3.new(-v.x, -v.y, -v.z)
end

function Vector3.__eq(a, b)
    return a.x == b.x and a.y == b.y and a.z == b.z
end

function Vector3.__tostring(v)
    return string.format("%.3f, %.3f, %.3f", v.x, v.y, v.z)
end

function Vector3:Dot(other)
    return self.x * other.x + self.y * other.y + self.z * other.z
end

function Vector3:Cross(other)
    return Vector3.new(
        self.y * other.z - self.z * other.y,
        self.z * other.x - self.x * other.z,
        self.x * other.y - self.y * other.x
    )
end

function Vector3:Lerp(other, alpha)
    return self + (other - self) * alpha
end

Vector3.zero = Vector3.new(0, 0, 0)
Vector3.one = Vector3.new(1, 1, 1)
Vector3.xAxis = Vector3.new(1, 0, 0)
Vector3.yAxis = Vector3.new(0, 1, 0)
Vector3.zAxis = Vector3.new(0, 0, 1)

local vector3_fields = {
    Magnitude = function(v)
        return math.sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
    end,
    Unit = function(v)
        local m = v.Magnitude
        if m == 0 then return Vector3.zero end
        return Vector3.new(v.x / m, v.y / m, v.z / m)
    end,
    X = function(v) return v.x end,
    Y = function(v) return v.y end,
    Z = function(v) return v.z end,
}

local vector3_methods = {}
for key, value in pairs(Vector3) do
    if type(value) == "function" then vector3_methods[key] = value end
end

function Vector3.__index(v, key)
    local field = vector3_fields[key]
    if field then return field(v) end
    return vector3_methods[key]
end

Color3 = {}
Color3.__index = Color3

function Color3.new(r, g, b)
    return setmetatable({ R = r or 0, G = g or 0, B = b or 0 }, Color3)
end

function Color3.fromRGB(r, g, b)
    return Color3.new((r or 0) / 255, (g or 0) / 255, (b or 0) / 255)
end

function Color3.__eq(a, b)
    return a.R == b.R and a.G == b.G and a.B == b.B
end

function Color3.__tostring(c)
    return string.format("%.3f, %.3f, %.3f", c.R, c.G, c.B)
end

function Color3:Lerp(other, alpha)
    return Color3.new(
        self.R + (other.R - self.R) * alpha,
        self.G + (other.G - self.G) * alpha,
        self.B + (other.B - self.B) * alpha
    )
end
