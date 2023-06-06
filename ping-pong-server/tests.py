import pytest
from http3_server import SessionTicketStore, WebTransportHandler, HttpRequestHandler
from dataclasses import dataclass
import asyncio

@dataclass
class Ticket:
    ticket: bytes

@dataclass
class H3Event:
    data: bytes

def test_session_ticket_store_add():
    store = SessionTicketStore()
    store.add(Ticket(b"a"))

    assert store.tickets.get(b'a') is not None

def test_session_ticket_store_pop():
    store = SessionTicketStore()
    store.tickets = {b"a": 1, b"b": 2}

    store.pop(b"a")

    assert store.tickets.get(b"a") is None

def test_web_transport_handler():
    wth = WebTransportHandler(connection=None, scope=None, stream_id=None, transmit=None)

    with pytest.raises(asyncio.QueueEmpty):
        assert wth.queue.get_nowait()

def test_http_handler():
    hth = HttpRequestHandler(authority=None,connection=None, protocol=None, scope=None, stream_id=None, transmit=None, stream_ended=None)

    with pytest.raises(asyncio.QueueEmpty):
        assert hth.queue.get_nowait()