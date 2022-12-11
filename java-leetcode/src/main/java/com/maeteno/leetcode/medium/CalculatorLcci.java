package com.maeteno.leetcode.medium;

import lombok.extern.slf4j.Slf4j;

import java.util.Objects;
import java.util.concurrent.LinkedBlockingDeque;

/**
 * 给定一个包含正整数、加(+)、减(-)、乘(*)、除(/)的算数表达式(括号除外)，计算其结果。
 * <p>
 * 表达式仅包含非负整数，+， - ，*，/ 四种运算符和空格  。 整数除法仅保留整数部分。
 *
 * @author SL.F
 */
@Slf4j
public class CalculatorLcci {

    private char[] operator1 = {'+', '-'};
    private char[] operator2 = {'*', '/'};

    public int calculate(String str) {
        char[] chars = str.toCharArray();
        LinkedBlockingDeque<String> deque = new LinkedBlockingDeque<>();
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
        deque.push(cache);

        LinkedBlockingDeque<String> deque2 = new LinkedBlockingDeque<>();

        cache = "";
        while (!deque.isEmpty()) {
            String tmp = deque.poll();
            switch (tmp) {
                case "+":
                case "-":
                    deque2.push(cache);
                    deque2.push(tmp);
                    break;
                case "*":
                    int i1 = Integer.parseInt(Objects.requireNonNull(deque.poll())) * Integer.parseInt(cache);
                    deque.push(String.valueOf(i1));
                    break;
                case "/":
                    int i2 = Integer.parseInt(Objects.requireNonNull(deque.poll())) / Integer.parseInt(cache);
                    deque.push(String.valueOf(i2));
                    break;
                default:
                    cache = tmp;
            }
        }

        deque2.push(cache);

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

    private static String push(LinkedBlockingDeque<String> deque, String item, String operator) {
        deque.push(item);
        deque.push(operator);
        return "";
    }
}
