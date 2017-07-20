require 'ffi'

module Example
  def self.new()
    Foo::Binding.new()
  end

  class Foo < FFI::AutoPointer
    def self.release(ptr)
      Binding.free(ptr)
    end

    def take_as_struct
      Binding.take_as_struct(self)
    end

    def take_as_trait
      Binding.take_as_trait(self)
    end

    module Binding
      extend FFI::Library
      ffi_lib 'target/debug/libffi_playground.dylib'

      attach_function :release, :foo_free, [Foo], :void
      attach_function :new, :foo_new, [], Foo
      attach_function :take_as_struct, :foo_take_as_struct, [Foo], :void
      attach_function :take_as_trait, :foo_take_as_trait, [Foo], :void
    end
  end
end

e = Example.new()
e.take_as_struct()
e.take_as_trait()
