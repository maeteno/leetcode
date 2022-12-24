package com.maeteno.leetcode.concurrency;

import java.util.concurrent.atomic.AtomicInteger;

public class Foo {
    AtomicInteger tag = new AtomicInteger(0);

    public Foo() {

    }

    public void first(Runnable printFirst) throws InterruptedException {

        // printFirst.run() outputs "first". Do not change or remove this line.
        printFirst.run();
        tag.incrementAndGet();
    }

    public void second(Runnable printSecond) throws InterruptedException {

        // printSecond.run() outputs "second". Do not change or remove this line.

        while (true) {
            if (tag.get() == 1) {
                break;
            }
        }

        printSecond.run();

        tag.incrementAndGet();
    }

    public void third(Runnable printThird) throws InterruptedException {
        while (true) {
            if (tag.get() == 2) {
                break;
            }
        }

        // printThird.run() outputs "third". Do not change or remove this line.
        printThird.run();
    }
}