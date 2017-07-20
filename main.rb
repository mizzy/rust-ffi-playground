require 'ffi'

module Sample
  extend FFI::Library

  ffi_lib 'target/debug/libffi_playground.dylib'

  class SampleStruct < FFI::Struct
    layout(
           :int, :int,
           :str, :string,
           )
  end

  attach_function :new, [], SampleStruct.by_value
end

s = Sample.new()
puts s[:int]
puts s[:str]
