module Codec where

import qualified Data.ByteString as BS
import qualified Data.ByteString.Char8 as C8
import Data.Word
import Data.Int
import Data.Bits
import Unsafe.Coerce (unsafeCoerce)

data Value
    = VInt Int64
    | VFloat Double
    | VBool Bool
    | VString String
    | VBytes BS.ByteString

decodeValues :: BS.ByteString -> [Value]
decodeValues bs
    | BS.null bs = []
    | otherwise =
        let (value, rest) = decodeOne bs
         in value : decodeValues rest

decodeOne :: BS.ByteString -> (Value, BS.ByteString)
decodeOne bs =
    case BS.uncons bs of
        Nothing -> error "Empty input"
        Just (tag, rest)
            | tag == 0 ->
                let (payload, remaining) = BS.splitAt 8 rest
                 in (VInt (fromIntegral (bytesToWord64 payload)), remaining)

            | tag == 1 ->
                let (payload, remaining) = BS.splitAt 8 rest
                 in (VFloat (unsafeCoerce (bytesToWord64 payload)), remaining)

            | tag == 2 ->
                let (payload, remaining) = BS.splitAt 1 rest
                 in case BS.unpack payload of
                        [b] -> (VBool (b /= 0), remaining)
                        _ -> error "Invalid Bool encoding"

            | tag == 3 ->
                let (lenBytes, afterLen) = BS.splitAt 8 rest
                    strLen = fromIntegral (bytesToWord64 lenBytes)
                    (payload, remaining) = BS.splitAt strLen afterLen
                 in (VString (C8.unpack payload), remaining)

            | tag == 4 ->
                let (lenBytes, afterLen) = BS.splitAt 8 rest
                    rawLen = fromIntegral (bytesToWord64 lenBytes)
                    (payload, remaining) = BS.splitAt rawLen afterLen
                 in (VBytes payload, remaining)

            | otherwise ->
                error ("Invalid type tag: " ++ show tag)

encodeInt :: Int64 -> BS.ByteString
encodeInt x =
    BS.pack (0 : word64ToBytes (fromIntegral x))

encodeFloat :: Double -> BS.ByteString
encodeFloat x =
    BS.pack (1 : word64ToBytes (unsafeCoerce x))

encodeBool :: Bool -> BS.ByteString
encodeBool b =
    BS.pack [2, if b then 1 else 0]

encodeString :: String -> BS.ByteString
encodeString s =
    let bytes = C8.pack s
        lenBytes = word64ToBytes (fromIntegral (BS.length bytes))
     in BS.concat [BS.pack [3], BS.pack lenBytes, bytes]

encodeBytes :: BS.ByteString -> BS.ByteString
encodeBytes b =
    let lenBytes = word64ToBytes (fromIntegral (BS.length b))
     in BS.concat [BS.pack [4], BS.pack lenBytes, b]

word64ToBytes :: Word64 -> [Word8]
word64ToBytes w =
    [ fromIntegral ( w              .&. 0xff)
    , fromIntegral ((w `shiftR` 8)  .&. 0xff)
    , fromIntegral ((w `shiftR` 16) .&. 0xff)
    , fromIntegral ((w `shiftR` 24) .&. 0xff)
    , fromIntegral ((w `shiftR` 32) .&. 0xff)
    , fromIntegral ((w `shiftR` 40) .&. 0xff)
    , fromIntegral ((w `shiftR` 48) .&. 0xff)
    , fromIntegral ((w `shiftR` 56) .&. 0xff)
    ]

bytesToWord64 :: BS.ByteString -> Word64
bytesToWord64 bs =
    case BS.unpack bs of
        [b0, b1, b2, b3, b4, b5, b6, b7] ->
              fromIntegral b0
            .|. (fromIntegral b1 `shiftL` 8)
            .|. (fromIntegral b2 `shiftL` 16)
            .|. (fromIntegral b3 `shiftL` 24)
            .|. (fromIntegral b4 `shiftL` 32)
            .|. (fromIntegral b5 `shiftL` 40)
            .|. (fromIntegral b6 `shiftL` 48)
            .|. (fromIntegral b7 `shiftL` 56)
        _ -> error "Expected exactly 8 bytes"