#!/usr/bin/env python3
"""runme.py
MCVE showing showing type weirdness in Python/PyO3.
(C) 2020 Benjamin Naecker
"""

import os
import platform


def link_libraries():
    names = ("libproducer", "libconsumer")
    lib_extension = ".so" if platform.system() == "Linux" else ".dylib"
    base_path = "./target/debug/"
    for name in names:
        source = os.path.join(base_path, f"{name}{lib_extension}")
        new_name = name.replace("lib", "")
        dest = f"./{new_name}.so"
        if os.path.exists(dest):
            os.remove(dest)
        os.symlink(source, dest)


if __name__ == "__main__":
    link_libraries()
    import producer
    import consumer

    obj = producer.MyClass(10)
    consumer.print_type_info(obj)
    consumer.convert_to_myclass(obj)
    consumer.print_data(obj)
