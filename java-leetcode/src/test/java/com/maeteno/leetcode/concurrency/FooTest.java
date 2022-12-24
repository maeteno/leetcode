package com.maeteno.leetcode.concurrency;

import lombok.extern.slf4j.Slf4j;
import org.junit.Test;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

@Slf4j
public class FooTest {

    @Test
    public void test_run() {
        ExecutorService pool = Executors.newFixedThreadPool(3);

        Foo foo = new Foo();

        pool.submit(() -> {
            try {
                foo.second(() -> log.info("second"));
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        });

        pool.submit(() -> {
            try {
                foo.first(() -> log.info("first"));
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        });

        pool.submit(() -> {
            try {
                foo.third(() -> log.info("third"));
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        });
    }
}