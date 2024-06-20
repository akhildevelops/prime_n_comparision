const std = @import("std");

fn is_prime(n: u32) bool {
    if (n == 2) {
        return true;
    }
    if (n == 1 or n % 2 == 0) {
        return false;
    }
    const sqrt = std.math.sqrt(n);
    var init: u32 = 3;
    while (init <= sqrt) : (init += 2) {
        if (n % init == 0) {
            return false;
        }
    }
    return true;
}

fn gen_primes(n_primes: u32, callback_s: anytype) !void {
    var counter: u32 = 0;
    var n: u32 = 0;
    while (counter < n_primes) : (n += 1) {
        if (is_prime(n)) {
            counter += 1;
            try callback_s.callback(counter, n);
        }
    }
}

const PrimeContext = struct {
    width: u32,
    start_time: *const std.time.Instant,
    buffered_writer: *std.io.BufferedWriter(4096, std.fs.File.Writer),

    pub fn callback(self: @This(), nth_prime: u32, prime_val: u32) !void {
        if (nth_prime % self.width == 0) {
            var end_time = std.time.Instant.now() catch unreachable;
            try std.fmt.format(self.buffered_writer.writer(), "{},{},{}\n", .{ nth_prime, prime_val, end_time.since(self.start_time.*) });
        }
    }
};

pub fn main() !void {
    var args = std.process.args();
    _ = args.next().?;
    const n = try std.fmt.parseInt(u32, args.next().?, 10);
    const width = try std.fmt.parseInt(u32, args.next().?, 10);
    const op_file = try std.fs.cwd().createFile("../benchmark_zig", .{});
    defer op_file.close();
    var bw = std.io.bufferedWriter(op_file.writer());
    const start = try std.time.Instant.now();
    const rc = PrimeContext{ .width = width, .start_time = &start, .buffered_writer = &bw };
    try gen_primes(n, rc);
    try bw.flush();
}
