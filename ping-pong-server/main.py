from starlette.applications import Starlette
from starlette.responses import Response
from starlette.routing import Route
from starlette.types import Receive, Scope, Send


async def homepage(*args, **kwargs):
    """
    Simple homepage.
    """
    return Response(content=b"Server is running")


async def wt(receive: Receive, send: Send) -> None:
    """
    WebTransport echo endpoint.
    """
    # accept connection
    message = await receive()
    assert message["type"] == "webtransport.connect"
    await send({"type": "webtransport.accept"})

    # echo back received data
    while True:
        message = await receive()
        if message["type"] == "webtransport.datagram.receive":
            await send(
                {
                    "data": message["data"],
                    "type": "webtransport.datagram.send",
                }
            )
        elif message["type"] == "webtransport.stream.receive":
            data = message["data"]
            if data == b"ping":
                await send(
                    {
                        "data": b"pong",
                        "stream": message["stream"],
                        "type": "webtransport.stream.send",
                    }
                )


starlette = Starlette(
    routes=[
        Route("/", homepage),
    ]
)


async def app(scope: Scope, receive: Receive, send: Send) -> None:
    if scope["type"] == "webtransport" and scope["path"] == "/":
        await wt(receive, send)
    else:
        await starlette(scope, receive, send)
