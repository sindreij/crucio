# Crucio

Crucio is a test harness used to see how a webservice behaves when its dependencies has
some kind of fault.

## Installation

Rust nightly is required (until async/await is stable).

```
cargo install crucio
```

## Usage

You need to have a file with some content that used for healty responses.

```
crucio
```

Crucio listens to multiple ports. Each port will emulate one failure mode. Set your application
to connecto to one of the ports to check how it responds to that kind of failure mode.

When `basePort` is `10000` (the default), this is the ports and their failure modes.

### 10000 - Http Healthy

The control. Healthy response, it returns the input file given with no delay.

### 10001 - Http Slow

Waits 9 seconds before returning the file.

### 10002 - Http Slow Body

Return the HTTP header instantly, then waits 1 second between each byte in the body.

### 10003 - Http Random

Correct HTTP header, but the body is just a infinite stream of random bytes.

### 10004 - Http Random Text

Correct HTTP header, but the body is just a infinite stream of random alphanumeric characters.

### 10005 - Http Never

Will accept the response, but let it hang and never return anything.

### 10006 - Http Header but no body

Will accpet the response and return a header, but will just hang and never return a body.

### 10007 - TCP Echo

TCP Echo server. Will just return what it gets from the client.

### 10008 - TCP Drop

TCP server which will close the connections immediately after having accepted a connection.

### 10009 - TCP Hanging socket

TCP Server which drops the socket, which means it will never be closed. In effect this is the same as 10005.

### 10010 - TCP Never Accept

TCP Server which will never accept any connections.

### 10011 - TCP Random TCP

TCP Server which will just send a few random bytes and then close the socket.

### 10012 - Random Infinite TCP

TCP Server which will send a infinite stream of random bytes. Will never close the socket.

### 10013 - Http Random Sleep

Like `Http Slow`, but will wait for a random amount of time. The number of ms to sleep is LogNormal, which means that most request will return relatively fast, but some will take a long time.

### 10014 - Http Random Sleep with some errors

Like Http Random Sleep, but will fail 50% of the time, returning a 500.

### 10015 - Http Always Error

Will always return a 500 - internal server error.

### 10016 - Http Slow Error

Will always return a 500, but with a 9 second delay.
