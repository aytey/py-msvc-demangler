import doctest
import msvc_demangler


def load_tests(loader, tests, ignore):
    tests.addTests(doctest.DocTestSuite(msvc_demangler))
    return tests
