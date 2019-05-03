#!/bin/bash

# bindgen interop/Linux/include/DeckLinkAPI.h -o src/sdk.rs -- -x c++
bindgen libdecklink_c/include/decklink_c.h -o src/sdk.rs
