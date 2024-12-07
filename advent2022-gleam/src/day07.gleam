import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/result

// a Command is a list of strings; the first string is the command and all
// others are the output of that command
type Command =
  List(String)

type FileSystem {
  FileSystem(
    // map of full pathnames to file size
    file_sizes: map.Map(String, Int),
    // list of full pathnames
    dirs: List(String),
    // cwd is a list of path parts, accumulated in reverse order
    cwd: List(String),
  )
}

fn newfs() -> FileSystem {
  FileSystem(file_sizes: map.new(), dirs: [], cwd: [])
}

// Returns a slash-separated full path with trailing slash
fn dir_name(dir: List(String)) -> String {
  let str =
    dir
    |> list.reverse
    |> string.join("/")
  "/" <> str <> "/"
}

// Returns a slash-separated full path 
fn file_name(basename: String, dir: List(String)) -> String {
  dir_name(dir) <> basename
}

// Creates a directory inside fs.cwd
fn mkdir(fs: FileSystem, name: String) -> FileSystem {
  FileSystem(
    file_sizes: fs.file_sizes,
    dirs: [dir_name([name, ..fs.cwd]), ..fs.dirs],
    cwd: fs.cwd,
  )
}

// Adds a file inside fs.cwd
fn add_file(fs: FileSystem, name: String, size: Int) -> FileSystem {
  FileSystem(
    file_sizes: map.insert(fs.file_sizes, file_name(name, fs.cwd), size),
    dirs: fs.dirs,
    cwd: fs.cwd,
  )
}

fn dir_size(fs: FileSystem, dir: String) -> Int {
  fs.file_sizes
  |> map.keys
  |> list.fold(
    0,
    fn(acc, filename) {
      case string.starts_with(filename, dir) {
        False -> acc
        True ->
          acc + {
            fs.file_sizes
            |> map.get(filename)
            |> result.unwrap(0)
          }
      }
    },
  )
}

fn cd(fs: FileSystem, dir: String) -> FileSystem {
  case dir {
    ".." ->
      FileSystem(
        file_sizes: fs.file_sizes,
        dirs: fs.dirs,
        cwd: list.drop(fs.cwd, 1),
      )
    _ ->
      FileSystem(file_sizes: fs.file_sizes, dirs: fs.dirs, cwd: [dir, ..fs.cwd])
  }
}

fn parse_ls(fs: FileSystem, output: List(String)) -> FileSystem {
  output
  |> list.fold(
    fs,
    fn(fsacc, line) {
      case string.split(line, " ") {
        ["dir", dir] -> mkdir(fsacc, dir)
        [size_str, file] -> {
          assert Ok(size) = int.parse(size_str)
          add_file(fsacc, file, size)
        }
        _ -> fsacc
      }
    },
  )
}

fn run_commands(fs: FileSystem, commands: List(Command)) -> FileSystem {
  case commands {
    [] -> fs
    [command, ..rest] -> {
      assert [cmd, ..output] = command
      case cmd {
        "cd " <> dir -> cd(fs, dir)
        "ls" -> parse_ls(fs, output)
        _ -> fs
      }
      |> run_commands(rest)
    }
  }
}

fn input() -> List(Command) {
  assert Ok(str) =
    "inputs/day07.txt"
    |> file.read()

  assert [_, ..cmds] =
    str
    |> string.split("\n$ ")
    |> list.map(string.split(_, "\n"))

  cmds
}

pub fn part1() {
  let fs = run_commands(newfs(), input())

  fs.dirs
  |> list.map(fn(dir) { dir_size(fs, dir) })
  |> list.filter(fn(size) { size < 100000 })
  |> list.fold(0, fn(acc, size) { acc + size })
  |> int.to_string
  |> io.println
}

pub fn part2() {
  let fs = run_commands(newfs(), input())

  let total_size =
    fs.file_sizes
    |> map.values
    |> list.fold(0, fn(acc, size) { acc + size })

  let free_space = 70000000 - total_size
  let need_to_free = 30000000 - free_space

  assert [size] =
    fs.dirs
    |> list.map(fn(dir) { dir_size(fs, dir) })
    |> list.filter(fn(size) { size >= need_to_free })
    |> list.sort(fn(a, b) { int.compare(a, b) })
    |> list.take(1)

  size
  |> int.to_string
  |> io.println
}
