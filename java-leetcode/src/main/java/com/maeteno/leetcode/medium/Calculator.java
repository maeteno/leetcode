package com.maeteno.leetcode.medium;

import lombok.extern.slf4j.Slf4j;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.Objects;

/**
 * <a href="https://leetcode.cn/problems/calculator-lcci/">面试题 16.26. 计算器</a>
 * 给定一个包含正整数、加(+)、减(-)、乘(*)、除(/)的算数表达式(括号除外)，计算其结果。
 * <p>
 * 表达式仅包含非负整数，+， - ，*，/ 四种运算符和空格  。 整数除法仅保留整数部分。
 *
 * @author SL.F
 */
@Slf4j
public class Calculator {

    public int calculate(String str) {
        char[] chars = str.toCharArray();
        Deque<String> deque = new ArrayDeque<>();

        String cache = "";
        for (char c : chars) {
            switch (c) {
                case ' ':
                    break;
                case '+':
                    cache = push(deque, cache, "+");
                    break;
                case '-':
                    cache = push(deque, cache, "-");
                    break;
                case '*':
                    cache = push(deque, cache, "*");
                    break;
                case '/':
                    cache = push(deque, cache, "/");
                    break;
                default:
                    cache = String.format("%s%c", cache, c);
            }
        }
        deque.addLast(cache);

        Deque<String> deque2 = new ArrayDeque<>();

        cache = "";
        while (!deque.isEmpty()) {
            String tmp = deque.poll();
            cache = switch (tmp) {
                case "+", "-" -> {
                    deque2.addLast(cache);
                    deque2.addLast(tmp);
                    yield "";
                }
                case "*" -> {
                    int i1 = Integer.parseInt(cache) * Integer.parseInt(Objects.requireNonNull(deque.poll()));
                    deque.push(String.valueOf(i1));
                    yield "";
                }
                case "/" -> {
                    int i2 = Integer.parseInt(cache) / Integer.parseInt(Objects.requireNonNull(deque.poll()));
                    deque.push(String.valueOf(i2));
                    yield "";
                }
                default -> tmp;
            };
        }

        deque2.addLast(cache);

        int reuslt = 0;
        while (!deque2.isEmpty()) {
            String tmp = deque2.poll();
            reuslt = switch (tmp) {
                case "+" -> reuslt + Integer.parseInt(Objects.requireNonNull(deque2.poll()));
                case "-" -> reuslt - Integer.parseInt(Objects.requireNonNull(deque2.poll()));
                default -> Integer.parseInt(tmp);
            };
        }

        return reuslt;
    }

    private static String push(Deque<String> deque, String item, String operator) {
        deque.addLast(item);
        deque.addLast(operator);
        return "";
    }
}
