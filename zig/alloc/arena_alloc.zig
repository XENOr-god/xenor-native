const std = @import("std");

pub const FixedArena = struct {
    buffer: []u8,
    cursor: usize = 0,

    pub fn init(buffer: []u8) FixedArena {
        return .{ .buffer = buffer };
    }

    pub fn reset(self: *FixedArena) void {
        self.cursor = 0;
    }

    pub fn alloc(self: *FixedArena, comptime T: type, count: usize) ![]T {
        const alignment = @alignOf(T);
        const bytes = @sizeOf(T) * count;
        const start = std.mem.alignForward(usize, self.cursor, alignment);
        const end = start + bytes;
        if (end > self.buffer.len) {
            return error.OutOfMemory;
        }

        self.cursor = end;
        const slice = self.buffer[start..end];
        return std.mem.bytesAsSlice(T, slice);
    }
};

test "arena allocation is deterministic for identical requests" {
    var storage: [128]u8 = undefined;
    var arena = FixedArena.init(storage[0..]);

    const first = try arena.alloc(u32, 4);
    arena.reset();
    const second = try arena.alloc(u32, 4);

    try std.testing.expect(@intFromPtr(first.ptr) == @intFromPtr(second.ptr));
}
