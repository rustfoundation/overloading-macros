use cpp::cpp;

#[test]
fn test_stdcpp_classes() {
    // In the same order as <https://github.com/rustfoundation/overloading-macros/issues/19>
    cpp! {{
        using namespace std;

        // Sequence containers
        #include <array>
        auto a = new array<int, 3>({1, 2, 3});

        #include <vector>
        auto v = new vector<int>({1, 2, 3});

        #include <version>
        #if __cpp_lib_hive >= 202502L
            #include <hive>
            auto h = new hive<int>({1, 2, 3});
            #error "hive is now supported, update the tests"
        #endif

        #include <deque>
        auto d = new deque<int>({1, 2, 3});

        #include <forward_list>
        auto f = new forward_list<int>({1, 2, 3});

        #include <list>
        auto l = new list<int>({1, 2, 3});

        // Associative containers
        #include <set>
        auto s = new set<int>({1, 2, 3});

        #include <map>
        auto m = new multimap<int, int>({{1, 2}, {3, 4}});

        #include <unordered_map>
        auto um = new unordered_map<int, int>({{1, 2}, {3, 4}});

        #include <unordered_set>
        auto us = new unordered_multiset<int>({1, 2, 3});

        // Strings
        #include <string>
        auto str = new string("hello");

        #include <string_view>
        // hello in ascii encoded as u32
        char32_t hello[] = {0x68, 0x65, 0x6c, 0x6c, 0x6f};
        auto str32_view = new u32string_view((char32_t*)hello);

        // Numeric
        #include <complex>
        auto c = new complex<float>(1.0f, 2.0f);

        // Chrono
        #include <chrono>
        using namespace chrono;
        auto tp = new time_point<system_clock>(system_clock::now());

        // Container adapters & views
        #include <stack>
        auto st = new stack<int>({1, 2, 3});

        #include <queue>
        auto q = new priority_queue<int>(less<int>(), {1, 2, 3});

        #include <span>
        int base[]{1, 2, 3};
        auto sp = new span<int, 3>{base};
    }}
}
